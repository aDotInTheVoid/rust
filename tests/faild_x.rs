const x: () = Crate {
    name: "fail",
    version: None,
    src: Real(
        Named(
            "tests/fail.rs",
        ),
    ),
    module: Some(
        Item {
            name: None,
            kind: ModuleItem(
                Module {
                    items: [
                        Item {
                            name: Some(
                                "inner",
                            ),
                            kind: StrippedItem(
                                ModuleItem(
                                    Module {
                                        items: [
                                            Item {
                                                name: Some(
                                                    "Public",
                                                ),
                                                kind: StructItem(
                                                    Struct,
                                                ),
                                                visibility: Public,
                                                def_id: "DefId(0:2 ~ fail[8787]::inner::Public)",
                                            },
                                        ],
                                        is_crate: false,
                                    },
                                ),
                            ),
                            visibility: Restricted(
                                "DefId(0:0 ~ fail[8787])",
                            ),
                            def_id: "DefId(0:1 ~ fail[8787]::inner)",
                        },
                        Item {
                            name: Some(
                                "Reexported",
                            ),
                            kind: StructItem(
                                Struct,
                            ),
                            visibility: Public,
                            def_id: "DefId(0:2 ~ fail[8787]::inner::Public)",
                        },
                    ],
                    is_crate: true,
                },
            ),
            visibility: Public,
            def_id: "DefId(0:0 ~ fail[8787])",
        },
    ),
    externs: [],
    primitives: [],
    external_traits: RefCell {
        value: {},
    },
    masked_crates: {},
    collapsed: true,
};