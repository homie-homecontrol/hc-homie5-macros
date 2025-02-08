use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemStruct};

#[proc_macro_attribute]
pub fn homie_device(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a struct
    let mut input = parse_macro_input!(item as ItemStruct);

    // Ensure the struct has named fields
    let fields = match &mut input.fields {
        Fields::Named(fields_named) => &mut fields_named.named,
        _ => {
            return syn::Error::new_spanned(
                input,
                "implement_homie_device only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        }
    };

    // Add the required fields for HomieDevice
    fields.push(syn::parse_quote! { device_ref: DeviceRef });
    fields.push(syn::parse_quote! { status: HomieDeviceStatus });
    fields.push(syn::parse_quote! { device_desc: HomieDeviceDescription });
    fields.push(syn::parse_quote! { homie_proto: Homie5DeviceProtocol });
    fields.push(syn::parse_quote! { homie_client: HomieMQTTClient });

    // Extract the struct name
    let struct_name = &input.ident;

    // Generate the necessary use statements
    let use_statements = quote! {
        use hc_homie5::{HomieDeviceCore, HomieMQTTClient};
        use homie5::{
            device_description::HomieDeviceDescription, Homie5DeviceProtocol, HomieDeviceStatus,
            HomieDomain, HomieID, DeviceRef,
        };
    };

    // Generate the default implementation for the HomieDevice trait
    let trait_impl = quote! {
        impl HomieDeviceCore for #struct_name {

            fn homie_domain(&self) -> &HomieDomain {
                self.device_ref.homie_domain()
            }

            fn homie_id(&self) -> &HomieID {
                self.device_ref.device_id()
            }

            fn device_ref(&self) -> &DeviceRef {
                &self.device_ref
            }

            fn description(&self) -> &HomieDeviceDescription {
                &self.device_desc
            }

            fn client(&self) -> &HomieMQTTClient {
                &self.homie_client
            }

            fn homie_proto(&self) -> &Homie5DeviceProtocol {
                &self.homie_proto
            }

            fn state(&self) -> HomieDeviceStatus {
                self.status
            }

            fn set_state(&mut self, state: HomieDeviceStatus) {
                self.status = state;
            }

        }
    };

    // Combine the modified struct and the trait implementation
    let expanded = quote! {
        #use_statements

        #input

        #trait_impl
    };

    expanded.into()
}
