// pub trait MyBundle: Bundle {
//     fn get_components(&self);
// }

// pub struct TemplateBundle<'a> {
//     components: Vec<(StorageType, &'a dyn Reflect)>,
// }
//
// impl<'a> From<&'a Template> for TemplateBundle<'a> {
//     fn from(value: &'a Template) -> Self {
//         let mut components = Vec::new();
//         for (storage_type, component) in value.components.values() {
//             components.push((*storage_type, component.as_ref()));
//         }
//         TemplateBundle { components }
//     }
// }
//
// impl<'a> DynamicBundle for TemplateBundle<'a> {
//     fn get_components(self, func: &mut impl FnMut(StorageType, OwningPtr<'_>)) {
//         for (storage_type, component) in self.components {
//             let _= component.downcast();
//
//             func(storage_type, unsafe {
//                 OwningPtr::new(NonNull::from(component).cast())
//             });
//         }
//     }
// }
