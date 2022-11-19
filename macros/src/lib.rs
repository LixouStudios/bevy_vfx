use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(VFXPipe)]
pub fn item_data_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &ast.ident;
    let generics = ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics bevy_vfx::prelude::VFXPipe for #name #ty_generics #where_clause {
            fn image(&self) -> bevy::prelude::Handle<bevy::prelude::Image> {
                self.image.clone_weak()
            }

            fn is_enabled(&self) -> bool {
                self.enabled
            }

            fn set_enabled(&mut self, enabled: bool) {
                self.enabled = enabled;
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
    gen.into()
}
