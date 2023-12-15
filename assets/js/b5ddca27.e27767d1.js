"use strict";(self.webpackChunkcddl_codegen_documentation=self.webpackChunkcddl_codegen_documentation||[]).push([[258],{3905:(e,t,n)=>{n.d(t,{Zo:()=>l,kt:()=>m});var r=n(7294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var d=r.createContext({}),p=function(e){var t=r.useContext(d),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},l=function(e){var t=p(e.components);return r.createElement(d.Provider,{value:t},e.children)},c="mdxType",y={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},u=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,d=e.parentName,l=i(e,["components","mdxType","originalType","parentName"]),c=p(n),u=a,m=c["".concat(d,".").concat(u)]||c[u]||y[u]||o;return n?r.createElement(m,s(s({ref:t},l),{},{components:n})):r.createElement(m,s({ref:t},l))}));function m(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,s=new Array(o);s[0]=u;var i={};for(var d in t)hasOwnProperty.call(t,d)&&(i[d]=t[d]);i.originalType=e,i[c]="string"==typeof e?e:a,s[1]=i;for(var p=2;p<o;p++)s[p]=n[p];return r.createElement.apply(null,s)}return r.createElement.apply(null,n)}u.displayName="MDXCreateElement"},9488:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>d,contentTitle:()=>s,default:()=>y,frontMatter:()=>o,metadata:()=>i,toc:()=>p});var r=n(7462),a=(n(7294),n(3905));const o={sidebar_position:3},s="Generating Keys and Addresses",i={unversionedId:"modules/crypto/generating_keys",id:"modules/crypto/generating_keys",title:"Generating Keys and Addresses",description:"BIP32 Keys",source:"@site/docs/modules/crypto/generating_keys.mdx",sourceDirName:"modules/crypto",slug:"/modules/crypto/generating_keys",permalink:"/modules/crypto/generating_keys",draft:!1,tags:[],version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3},sidebar:"tutorialSidebar",previous:{title:"Crypto",permalink:"/category/crypto"},next:{title:"crypto",permalink:"/modules/crypto/"}},d={},p=[{value:"BIP32 Keys",id:"bip32-keys",level:2},{value:"BIP39 Entropy",id:"bip39-entropy",level:2},{value:"Use in Addresses",id:"use-in-addresses",level:2},{value:"Other Key Types",id:"other-key-types",level:2}],l={toc:p},c="wrapper";function y(e){let{components:t,...n}=e;return(0,a.kt)(c,(0,r.Z)({},l,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h1",{id:"generating-keys-and-addresses"},"Generating Keys and Addresses"),(0,a.kt)("h2",{id:"bip32-keys"},"BIP32 Keys"),(0,a.kt)("p",null,"There are two main categories of keys in this library. There are the raw ",(0,a.kt)("inlineCode",{parentName:"p"},"PublicKey")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"PrivateKey")," which are used for cryptographically signing/verifying, and ",(0,a.kt)("inlineCode",{parentName:"p"},"BIP32PrivateKey"),"/",(0,a.kt)("inlineCode",{parentName:"p"},"BIP32PublicKey")," which in addition to this have the ability to derive additional keys from them following the ",(0,a.kt)("a",{parentName:"p",href:"https://en.bitcoin.it/wiki/BIP_0032"},"BIP32 derivation scheme")," variant called BIP32-Ed25519, which will be referred to as BIP32 for brevity. We use the ",(0,a.kt)("a",{parentName:"p",href:"https://en.bitcoin.it/wiki/BIP_0044"},"BIP44 spec")," variant for Ed25519 as well for the derivation paths using 1852 or 44 as the purpose consant and 1815 for the coin type depending on address type. See ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/input-output-hk/implementation-decisions/pull/18"},"this doc")," for more details."),(0,a.kt)("p",null,"This is demonstrated with the below code"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-javascript"},'function harden(num: number): number {\n  return 0x80000000 + num;\n}\n\n\nconst rootKey = CardanoWasm.BIP32PrivateKey.from_bech32("xprv17qx9vxm6060qjn5fgazfue9nwyf448w7upk60c3epln82vumg9r9kxzsud9uv5rfscxp382j2aku254zj3qfx9fx39t6hjwtmwq85uunsd8x0st3j66lzf5yn30hwq5n75zeuplepx8vxc502txx09ygjgx06n0p");\nconst accountKey = rootKey\n  .derive(harden(1852)) // purpose\n  .derive(harden(1815)) // coin type\n  .derive(harden(0)); // account #0\n\nconst utxoPubKey = accountKey\n  .derive(0) // external\n  .derive(0)\n  .to_public();\n\nconst stakeKey = accountKey\n  .derive(2) // chimeric\n  .derive(0)\n  .to_public();\n')),(0,a.kt)("h2",{id:"bip39-entropy"},"BIP39 Entropy"),(0,a.kt)("p",null,"To generate a ",(0,a.kt)("inlineCode",{parentName:"p"},"BIP32PrivateKey")," from a BIP39 recovery phrase it must be first converted to entropy following the BIP39 protocol(). This library does not directly handle that, but once entropy is created it is possible to use ",(0,a.kt)("inlineCode",{parentName:"p"},"Bip32PrivateKey.from_bip39_entropy(entropy, password)"),". For more information see the ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/cardano-foundation/CIPs/pull/3"},"CIP3")," Cardano improvement proposal. The code below uses the ",(0,a.kt)("inlineCode",{parentName:"p"},"bip39")," npm package to generate a root ",(0,a.kt)("inlineCode",{parentName:"p"},"BIP32PrivateKey")," from a BIP39 mnemonic."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-javascript"},'import { mnemonicToEntropy } from \'bip39\';\n\nconst entropy = mnemonicToEntropy(\n  [ "test", "walk", "nut", "penalty", "hip", "pave", "soap", "entry", "language", "right", "filter", "choice" ].join(\' \')\n);\n\nconst rootKey = CardanoWasm.Bip32PrivateKey.from_bip39_entropy(\n  Buffer.from(entropy, \'hex\'),\n  Buffer.from(\'\'),\n);\n')),(0,a.kt)("h2",{id:"use-in-addresses"},"Use in Addresses"),(0,a.kt)("p",null,"Once we have reached the desired derivation path, we must convert the ",(0,a.kt)("inlineCode",{parentName:"p"},"BIP32PrivateKey")," or ",(0,a.kt)("inlineCode",{parentName:"p"},"BIP32PublicKey")," to a ",(0,a.kt)("inlineCode",{parentName:"p"},"PrivateKey")," or ",(0,a.kt)("inlineCode",{parentName:"p"},"PublicKey")," by calling ",(0,a.kt)("inlineCode",{parentName:"p"},".to_raw_key()")," on them with the exception of Byron addresses.\nFor example, to create an address using the ",(0,a.kt)("inlineCode",{parentName:"p"},"utxoPubKey")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"stakeKey")," in the first example, we can do:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-javascript"},"// base address with staking key\nconst baseAddr = CardanoWasm.BaseAddress.new(\n  CardanoWasm.NetworkInfo.mainnet().network_id(),\n  CardanoWasm.StakeCredential.from_keyhash(utxoPubKey.to_raw_key().hash()),\n  CardanoWasm.StakeCredential.from_keyhash(stakeKey.to_raw_key().hash()),\n);\n\n// enterprise address without staking ability, for use by exchanges/etc\nconst enterpriseAddr = CardanoWasm.EnterpriseAddress.new(\n  CardanoWasm.NetworkInfo.mainnet().network_id(),\n  CardanoWasm.StakeCredential.from_keyhash(utxoPubKey.to_raw_key().hash())\n);\n\n// pointer address - similar to Base address but can be shorter, see formal spec for explanation\nconst ptrAddr = CardanoWasm.PointerAddress.new(\n  CardanoWasm.NetworkInfo.mainnet().network_id(),\n  CardanoWasm.StakeCredential.from_keyhash(utxoPubKey.to_raw_key().hash()),\n  CardanoWasm.Pointer.new(\n    100, // slot\n    2,   // tx index in slot\n    0    // cert indiex in tx\n  )\n);\n\n// reward address - used for withdrawing accumulated staking rewards\nconst rewardAddr = CardanoWasm.RewardAddress.new(\n  CardanoWasm.NetworkInfo.mainnet().network_id(),\n  CardanoWasm.StakeCredential.from_keyhash(stakeKey.to_raw_key().hash())\n);\n\n// bootstrap address - byron-era addresses with no staking rights\nconst byronAddr = CardanoWasm.ByronAddress.icarus_from_key(\n  utxoPubKey, // Ae2* style icarus address\n  CardanoWasm.NetworkInfo.mainnet().protocol_magic()\n);\n")),(0,a.kt)("p",null,"Note that the byron-era address can only be created in this library from icarus-style addresses that start in ",(0,a.kt)("inlineCode",{parentName:"p"},"Ae2")," and that Daedalus-style addresses starting in ",(0,a.kt)("inlineCode",{parentName:"p"},"Dd")," are not directly supported."),(0,a.kt)("p",null,"These are all address variant types with information specific to its address type. There is also an ",(0,a.kt)("inlineCode",{parentName:"p"},"Address")," type which represents any of those variants, which is the type use in most parts of the library. For example to create a ",(0,a.kt)("inlineCode",{parentName:"p"},"TransactionOutput")," manually we would have to first convert from one of the address variants by doing:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-javascript"},'const address = baseAddress.to_address();\n\nconst output = CardanoWasm.TransactionOutput(address, BigNum.from_str("365"));\n')),(0,a.kt)("p",null,"If the address is already a Shelley address in raw bytes or a bech32 string we can create it directly via:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-javascript"},'const addr = CardanoWasm.Address.from_bech32("addr1vyt3w9chzut3w9chzut3w9chzut3w9chzut3w9chzut3w9cj43ltf");\n\n')),(0,a.kt)("h2",{id:"other-key-types"},"Other Key Types"),(0,a.kt)("p",null,"Conversion between ",(0,a.kt)("inlineCode",{parentName:"p"},"cardano-cli")," 128-byte ",(0,a.kt)("inlineCode",{parentName:"p"},"XPrv")," keys and ",(0,a.kt)("inlineCode",{parentName:"p"},"BIP32PrivateKey")," is also supported:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-javascript"},"const bip32PrivateKey = CardanoWasm.BIP32PrivateKey.from_128_xprv(xprvBytes);\nassert(xprvBytes == CardanoWasm.BIP32PrivateKey.to_128_xprv());\n")),(0,a.kt)("p",null,"96-byte ",(0,a.kt)("inlineCode",{parentName:"p"},"XPrv")," keys are identical to ",(0,a.kt)("inlineCode",{parentName:"p"},"BIP32PrivateKey"),"s byte-wise and no conversion is needed.\nFor more details see ",(0,a.kt)("a",{parentName:"p",href:"https://docs.cardano.org/projects/cardano-node/en/latest/stake-pool-operations/keys_and_addresses.html"},"this document")," regarding legacy keys."),(0,a.kt)("p",null,"There is also ",(0,a.kt)("inlineCode",{parentName:"p"},"LegacyDaedalusPrivateKey")," which is used for creating witnesses for legacy Daedalus ",(0,a.kt)("inlineCode",{parentName:"p"},"Dd"),"-type addresses."))}y.isMDXComponent=!0}}]);