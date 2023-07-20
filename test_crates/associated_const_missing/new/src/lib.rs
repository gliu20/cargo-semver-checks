pub trait TraitWithConst {
    const MY_CONST: i32;
}

struct StructWithTraitWithConstWillRemoveImpl;

pub trait TraitWithConstWillChangeType {
    const MY_CONST: bool;
}
struct StructWithTraitWithConstWillChangeType;

impl TraitWithConstWillChangeType for StructWithTraitWithConstWillChangeType {
    const MY_CONST: bool = false;
}

struct StructWithConstWillRemoveConst;


trait PrivateTraitWithConst {
    const MY_CONST: i32;
}
struct StructWithPrivateTraitWithConstWillRemove;