// taken from cddl-codegen directly
use std::collections::{BTreeMap, BTreeSet};

use cddl::ast::*;

pub fn topological_rule_order<'a>(rules: &'a [&'a Rule<'a>]) -> Vec<&'a Rule<'a>> {
    let mut adj_list = BTreeMap::new();
    for cddl_rule in rules.iter() {
        let (ident, refs) = find_references(cddl_rule);
        adj_list.insert(ident.ident, (*cddl_rule, refs));
    }
    let mut unvisited = adj_list.keys().copied().collect::<BTreeSet<&str>>();
    let mut topo_order = Vec::new();
    let mut processing: BTreeSet<&'a str> = BTreeSet::new();
    while let Some(u) = unvisited.iter().next().copied() {
        dfs_visit(
            &mut topo_order,
            &mut unvisited,
            &mut processing,
            &adj_list,
            u,
        );
    }
    topo_order
}

fn dfs_visit<'a>(
    topo_order: &mut Vec<&'a Rule<'a>>,
    unvisited: &mut BTreeSet<&'a str>,
    processing: &mut BTreeSet<&'a str>,
    adj_list: &BTreeMap<&'a str, (&'a cddl::ast::Rule<'a>, Vec<&'a Identifier<'a>>)>,
    u: &'a str,
) {
    processing.insert(u);
    let (rule, neighbors) = adj_list.get(u).unwrap();
    for v in neighbors.iter() {
        if processing.contains(v.ident) {
            eprintln!("Recursive type: '{u}' / '{v}' - code will possibly need to be edited by hand to use Box/etc");
            continue;
        }
        if unvisited.contains(v.ident) {
            dfs_visit(topo_order, unvisited, processing, adj_list, v.ident);
        }
    }
    processing.remove(u);
    unvisited.remove(u);
    topo_order.push(rule);
}

fn find_references<'a>(cddl_rule: &'a Rule<'a>) -> (&'a Identifier, Vec<&'a Identifier<'a>>) {
    let mut refs = Vec::new();
    let ident = match cddl_rule {
        Rule::Type { rule, .. } => {
            rule.value
                .type_choices
                .iter()
                .for_each(|tc| find_refs_type1(&mut refs, &tc.type1));
            &rule.name
        }
        Rule::Group { rule, .. } => {
            assert_eq!(
                rule.generic_params, None,
                "{}: Generics not supported on plain groups",
                rule.name
            );
            match &rule.entry {
                cddl::ast::GroupEntry::InlineGroup { group, .. } => {
                    find_refs_group(&mut refs, group)
                }
                x => panic!("Group rule with non-inline group? {:?}", x),
            }
            &rule.name
        }
    };
    (ident, refs)
}

fn find_refs_type1<'a>(refs: &mut Vec<&'a Identifier<'a>>, type1: &'a Type1<'a>) {
    match &type1.type2 {
        Type2::Typename {
            ident,
            generic_args,
            ..
        } => {
            refs.push(ident);
            find_refs_generic_args(refs, generic_args);
        }
        Type2::ParenthesizedType { pt, .. } => pt
            .type_choices
            .iter()
            .for_each(|tc| find_refs_type1(refs, &tc.type1)),
        Type2::Map { group, .. } => find_refs_group(refs, group),
        Type2::Array { group, .. } => find_refs_group(refs, group),
        Type2::Unwrap {
            ident,
            generic_args,
            ..
        } => {
            refs.push(ident);
            find_refs_generic_args(refs, generic_args);
        }
        Type2::ChoiceFromInlineGroup { group, .. } => find_refs_group(refs, group),
        Type2::ChoiceFromGroup {
            ident,
            generic_args,
            ..
        } => {
            refs.push(ident);
            find_refs_generic_args(refs, generic_args);
        }
        Type2::TaggedData { t, .. } => t
            .type_choices
            .iter()
            .for_each(|tc| find_refs_type1(refs, &tc.type1)),
        _ => (),
    }
}

fn find_refs_group<'a>(refs: &mut Vec<&'a Identifier<'a>>, group: &'a Group<'a>) {
    for group_choice in group.group_choices.iter() {
        for (group_entry, _) in group_choice.group_entries.iter() {
            match group_entry {
                GroupEntry::InlineGroup { group, .. } => find_refs_group(refs, group),
                GroupEntry::TypeGroupname { ge, .. } => {
                    refs.push(&ge.name);
                    find_refs_generic_args(refs, &ge.generic_args);
                }
                GroupEntry::ValueMemberKey { ge, .. } => {
                    ge.entry_type
                        .type_choices
                        .iter()
                        .for_each(|tc| find_refs_type1(refs, &tc.type1));
                    match &ge.member_key {
                        Some(MemberKey::Type1 { t1, .. }) => find_refs_type1(refs, t1),
                        Some(MemberKey::NonMemberKey { .. }) => {
                            unimplemented!("Please open a github issue with repro steps")
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn find_refs_generic_args<'a>(
    refs: &mut Vec<&'a Identifier<'a>>,
    generic_arg: &'a Option<GenericArgs<'a>>,
) {
    if let Some(arg) = generic_arg {
        arg.args
            .iter()
            .for_each(|arg| find_refs_type1(refs, arg.arg.as_ref()));
    }
}
