; look at src/main.rs for the source code
; compile using 
; cargo rustc -- -O -g --emit=llvm-ir
; and find your .ll file in target/debug

; Generally, I think rustc handles monomorphization, so llvm will never see template types comming from rust in that sense.
; https://rustc-dev-guide.rust-lang.org/backend/monomorph.html
; However, they seem to be used for other information, se below


!4 = !{}


; 166 and 168 are used by the code below
!166 = !DIDerivedType(tag: DW\_TAG\_pointer\_type, name: "*mut u8", baseType: !41, size: 64, align: 64, dwarfAddressSpace: 0)
!41 = !DIBasicType(name: "u8", size: 8, encoding: DW_ATE_unsigned)

; Why do we have [usize; 3]? usize = pointer length, I would get [usize; 4], as in one entry per impl, but why 3?
!168 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "&[usize; 3]", baseType: !169, size: 64, align: 64, dwarfAddressSpace: 0)
!169 = !DICompositeType(tag: DW_TAG_array_type, baseType: !136, size: 192, align: 64, elements: !170)
!170 = !{!171}
!171 = !DISubrange(count: 3, lowerBound: 0)


; START HERE
!1094 = !DILocalVariable(name: "animal", scope: !1095, file: !1051, line: 54, type: !579, align: 8)

!579 = !DICompositeType(tag: DW_TAG_structure_type, name: "Box<dyn crate::Animal, alloc::alloc::Global>", size: 128, align: 64, elements: !581, templateParams: !584, identifier: "347..")
!581 = !{!582, !583}
!582 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", baseType: !166, size: 64, align: 64, flags: DIFlagArtificial)
!583 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", baseType: !168, size: 64, align: 64, offset: 64, flags: DIFlagArtificial)

; templateParams
!584 = !{!585, !590}

!585 = !DITemplateTypeParameter(name: "T", type: !586)
!586 = !DICompositeType(tag: DW_TAG_structure_type, name: "dyn crate::Animal", align: 8, elements: !587, templateParams: !4, identifier: "b26..")

; not sure when this one is usefull, an empty type? We don't use Option(Animal) here.
!590 = !DITemplateTypeParameter(name: "A", type: !542)
!542 = !DICompositeType(tag: DW_TAG_structure_type, name: "Global", align: 8, elements: !4, templateParams: !4, identifier: "c23..")


; dyn dwarf::Animal elements
!587 = !{!588, !589}
!588 = !DIDerivedType(tag: DW_TAG_member, name: "pointer", baseType: !166, size: 64, align: 64, flags: DIFlagArtificial)
!589 = !DIDerivedType(tag: DW_TAG_member, name: "vtable", baseType: !168, size: 64, align: 64, offset: 64, flags: DIFlagArtificial)




; So the order is: 
; 1) !DICompositeType(tag: DW_TAG_structure_type, name: "Box<dyn something, alloc::alloc::Global>") containing a struct { pointer, vtable }, both of type DW_TAG_member
; 2) 
;
;

