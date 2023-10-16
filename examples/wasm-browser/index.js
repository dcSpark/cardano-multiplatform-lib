import * as CML from "cml-chain-wasm";
import * as CIP25 from "cml-cip25-wasm";

function cip30Error(err) {
    alert(`cip30 err: ${JSON.stringify(err)}`);
}

function load() {
    if (window.cardano.flint != null) {
        window.cardano.flint.enable().then(
            (api) => {
                console.log("Flint connected");
                createMintButtons(api);
            },
            cip30Error
        );
    } else {
        alert("not connected - enable Flint and refresh");
    }
}

function createMintButtons(api) {
    const dapp = document.getElementById("dapp");
    dapp.textContent = "";
    const slotInput = document.createElement("input");
    slotInput.value = "1234567890";
    dapp.appendChild(slotInput);
    const imageInput = document.createElement("input");
    imageInput.value = "https://cardano.org/favicon-32x32.png";
    dapp.appendChild(imageInput);
    const assetNameInput = document.createElement("input");
    assetNameInput.value = "Asset Name Here";
    dapp.appendChild(assetNameInput);
    const mintButton = document.createElement("button");
    mintButton.textContent = "Mint!";
    mintButton.onclick = () => api
        .getChangeAddress()
        .then(
            (changeAddrHex) => api.getUtxos(CML.Value.from_coin(BigInt(3)).to_cbor_hex()).then(
            (utxos) => {
                try {
                    const changeAddr = CML.Address.from_hex(changeAddrHex);
                    const pubKey = changeAddr.payment_cred().as_pub_key();
                    const pkCheck = CML.NativeScript.new_script_pubkey(
                        pubKey
                    );
                    const slotCheck = CML.NativeScript.new_script_invalid_hereafter(
                        BigInt(slotInput.value)
                    );
                    const checks = CML.NativeScriptList.new();
                    checks.add(pkCheck);
                    // if I comment out the below line of code it works...
                    // with it it won't even if I put something absurdly high
                    // for a slot value
                    //checks.add(slotCheck);
                    const nftScript = CML.NativeScript.new_script_all(checks);
                    const nftHash = nftScript.hash();
                    const assetName = CML.AssetName.from_str(assetNameInput.value);
                    const cfg = CML.TransactionBuilderConfigBuilder.new()
                        .fee_algo(CML.LinearFee.new(BigInt(44), BigInt(155381)))
                        .coins_per_utxo_byte(BigInt(4310))
                        .pool_deposit(BigInt(500000000))
                        .key_deposit(BigInt(2000000))
                        .max_value_size(5000)
                        .max_tx_size(16384)
                        .ex_unit_prices(CML.ExUnitPrices.new(
                            CML.Rational.new(BigInt(577), BigInt(10_000)),
                            CML.Rational.new(BigInt(721), BigInt(10_000_000)),
                        ))
                        .cost_models(default_babbage_costmodels())
                        .collateral_percentage(150)
                        .max_collateral_inputs(3)
                        .build();
                    const txBuilder = CML.TransactionBuilder.new(cfg);
                    for (const utxoCborHex of utxos) {
                        const utxo = CML.TransactionUnspentOutput.from_cbor_hex(utxoCborHex);
                        const input = CML.SingleInputBuilder
                            .from_transaction_unspent_output(utxo)
                            .payment_key();
                        txBuilder.add_utxo(input);
                    }
                    const mintBuilder = CML.SingleMintBuilder.new_single_asset(
                        assetName, BigInt(1)
                    );
                    const vkeys = CML.Ed25519KeyHashList.new();
                    vkeys.add(pubKey);
                    const witnessInfo = CML.NativeScriptWitnessInfo.vkeys(vkeys);
                    const mintResult = mintBuilder.native_script(
                        nftScript,
                        witnessInfo,
                    );
                    txBuilder.add_mint(mintResult);
                    txBuilder.select_utxos(CML.CoinSelectionStrategyCIP2.LargestFirst);
                    const labelMd = CIP25.LabelMetadata.new(CIP25.CIP25Version.V2);
                    const mdDetails = CIP25.MetadataDetails.new(
                        CIP25.String64.new(assetNameInput.value),
                        CIP25.ChunkableString.from_string(imageInput.value),
                    );
                    labelMd.set(
                        CIP25.ScriptHash.from_raw_bytes(nftHash.to_raw_bytes()),
                        CIP25.AssetName.from_bytes(assetName.get()),
                        mdDetails
                    );
                    const metadata = CIP25.CIP25Metadata.new(labelMd);
                    const auxdata = CIP25.AuxiliaryData.new_shelley(metadata.to_metadata());
                    txBuilder.set_auxiliary_data(CML.AuxiliaryData.from_cbor_bytes(auxdata.to_cbor_bytes()));
                    const tx = txBuilder
                        .build(CML.ChangeSelectionAlgo.Default, changeAddr)
                        .build_unchecked();
                    console.log("Asking to sign...");
                    api.signTx(tx.to_cbor_hex()).then(
                        (witnessCborHex) => {
                            const witnessSet = CML.TransactionWitnessSet.from_cbor_hex(witnessCborHex);
                            witnessSet.add_all_witnesses(tx.witness_set());
                            const signedTx = CML.Transaction.new(
                                tx.body(),
                                witnessSet,
                                true,
                                tx.auxiliary_data()
                            );
                            api.submitTx(signedTx.to_cbor_hex()).then(
                                (hash) => {
                                    console.log(`tx sent - hash: ${hash}`);
                                },
                                cip30Error
                            )
                        },
                        cip30Error
                    );
                } catch (e) {
                    alert(`Invalid mint: ${e}`);
                }
            },
            cip30Error
        ),
        cip30Error
    );
    dapp.appendChild(mintButton);
}

function default_babbage_costmodels() {
    const costmodels = CML.CostModels.new();
    const v1 = [
        205665, 812, 1, 1, 1000, 571, 0, 1, 1000, 24177, 4, 1, 1000, 32, 117366, 10475,
        4, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 100,
        100, 23000, 100, 19537, 32, 175354, 32, 46417, 4, 221973, 511, 0, 1, 89141, 32,
        497525, 14068, 4, 2, 196500, 453240, 220, 0, 1, 1, 1000, 28662, 4, 2, 245000,
        216773, 62, 1, 1060367, 12586, 1, 208512, 421, 1, 187000, 1000, 52998, 1, 80436,
        32, 43249, 32, 1000, 32, 80556, 1, 57667, 4, 1000, 10, 197145, 156, 1, 197145, 156,
        1, 204924, 473, 1, 208896, 511, 1, 52467, 32, 64832, 32, 65493, 32, 22558, 32,
        16563, 32, 76511, 32, 196500, 453240, 220, 0, 1, 1, 69522, 11687, 0, 1, 60091, 32,
        196500, 453240, 220, 0, 1, 1, 196500, 453240, 220, 0, 1, 1, 806990, 30482, 4,
        1927926, 82523, 4, 265318, 0, 4, 0, 85931, 32, 205665, 812, 1, 1, 41182, 32,
        212342, 32, 31220, 32, 32696, 32, 43357, 32, 32247, 32, 38314, 32, 57996947, 18975,
        10,
    ];
    const v1_intlist = CML.IntList.new();
    for (let cost of v1) {
        v1_intlist.add(CML.Int.new(BigInt(cost)))
    }
    costmodels.set_plutus_v1(v1_intlist);
    const v2 = [
        205665, 812, 1, 1, 1000, 571, 0, 1, 1000, 24177, 4, 1, 1000, 32, 117366, 10475,
        4, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 23000, 100, 100,
        100, 23000, 100, 19537, 32, 175354, 32, 46417, 4, 221973, 511, 0, 1, 89141, 32,
        497525, 14068, 4, 2, 196500, 453240, 220, 0, 1, 1, 1000, 28662, 4, 2, 245000,
        216773, 62, 1, 1060367, 12586, 1, 208512, 421, 1, 187000, 1000, 52998, 1, 80436,
        32, 43249, 32, 1000, 32, 80556, 1, 57667, 4, 1000, 10, 197145, 156, 1, 197145, 156,
        1, 204924, 473, 1, 208896, 511, 1, 52467, 32, 64832, 32, 65493, 32, 22558, 32,
        16563, 32, 76511, 32, 196500, 453240, 220, 0, 1, 1, 69522, 11687, 0, 1, 60091, 32,
        196500, 453240, 220, 0, 1, 1, 196500, 453240, 220, 0, 1, 1, 1159724, 392670, 0, 2,
        806990, 30482, 4, 1927926, 82523, 4, 265318, 0, 4, 0, 85931, 32, 205665, 812, 1, 1,
        41182, 32, 212342, 32, 31220, 32, 32696, 32, 43357, 32, 32247, 32, 38314, 32,
        35892428, 10, 57996947, 18975, 10, 38887044, 32947, 10,
    ];
    const v2_intlist = CML.IntList.new();
    for (let cost of v2) {
        v2_intlist.add(CML.Int.new(BigInt(cost)))
    }
    costmodels.set_plutus_v2(v2_intlist);
    return costmodels;
}

load();