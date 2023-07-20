pub trait TraitWithConst {
    const MY_CONST: i32;
}

struct StructWithTraitWithConstWillRemoveImpl;
impl TraitWithConst for StructWithTraitWithConstWillRemoveImpl {
    const MY_CONST: i32 = 0;
}


pub trait TraitWithConstWillChangeType {
    const MY_CONST: i32;
}

struct StructWithTraitWithConstWillChangeType;
impl TraitWithConstWillChangeType for StructWithTraitWithConstWillChangeType {
    const MY_CONST: i32 = 0;
}

struct StructWithConstWillRemoveConst {
    MY_CONST: i32
}
