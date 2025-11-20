use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemEnum, ItemStruct, Path};

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

/// Usage:
///
/// #[homie_device_enum(crate::error::AppError)]
/// pub enum Devices {
///     Generic(GenericDevice),
///     Group(GroupDevice),
/// }
///
/// Requirements:
/// - Each variant is a tuple variant with exactly one field: Variant(InnerType)
/// - Each inner type implements `hc_homie5::HomieDevice`
/// - All inner types use the same error type `ErrorType`
#[proc_macro_attribute]
pub fn homie_device_enum(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Attribute is just a type path: #[homie_device_enum(ErrorType)]
    // or #[homie_device_enum(crate::error::AppError)]
    let error_ty: Path = parse_macro_input!(attr as Path);

    // The enum we’re attached to
    let input = parse_macro_input!(item as ItemEnum);
    let enum_name = &input.ident;
    let variants = &input.variants;

    // Enforce tuple variants with exactly one field
    for v in variants {
        match &v.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {}
            _ => {
                return syn::Error::new_spanned(
                    v,
                    "homie_device_enum only supports tuple variants with a single field, e.g. Variant(InnerType)",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    // Generate match arms for HomieDeviceCore
    let homie_domain_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! { #enum_name::#vident(inner) => inner.homie_domain(), }
    });

    let homie_id_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! { #enum_name::#vident(inner) => inner.homie_id(), }
    });

    let device_ref_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! { #enum_name::#vident(inner) => inner.device_ref(), }
    });

    let description_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! { #enum_name::#vident(inner) => inner.description(), }
    });

    let client_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! { #enum_name::#vident(inner) => inner.client(), }
    });

    let proto_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! { #enum_name::#vident(inner) => inner.homie_proto(), }
    });

    let state_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! { #enum_name::#vident(inner) => inner.state(), }
    });

    let set_state_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! { #enum_name::#vident(inner) => inner.set_state(state), }
    });

    // Generate match arms for HomieDevice methods we need to delegate
    let publish_prop_values_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! { #enum_name::#vident(inner) => inner.publish_property_values().await, }
    });

    let handle_set_command_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! {
            #enum_name::#vident(inner) => inner.handle_set_command(property, set_value).await,
        }
    });

    let publish_meta_arms = variants.iter().map(|v| {
        let vident = &v.ident;
        quote! { #enum_name::#vident(inner) => inner.publish_meta().await, }
    });

    // Final expanded code:
    //  - original enum
    //  - impl HomieDeviceCore for Enum
    //  - impl HomieDevice for Enum
    let expanded = quote! {
        #input

        impl hc_homie5::HomieDeviceCore for #enum_name {
            fn homie_domain(&self) -> &homie5::HomieDomain {
                match self {
                    #(#homie_domain_arms)*
                }
            }

            fn homie_id(&self) -> &homie5::HomieID {
                match self {
                    #(#homie_id_arms)*
                }
            }

            fn device_ref(&self) -> &homie5::DeviceRef {
                match self {
                    #(#device_ref_arms)*
                }
            }

            fn description(&self) -> &homie5::device_description::HomieDeviceDescription {
                match self {
                    #(#description_arms)*
                }
            }

            fn client(&self) -> &hc_homie5::HomieMQTTClient {
                match self {
                    #(#client_arms)*
                }
            }

            fn homie_proto(&self) -> &homie5::Homie5DeviceProtocol {
                match self {
                    #(#proto_arms)*
                }
            }

            fn state(&self) -> homie5::HomieDeviceStatus {
                match self {
                    #(#state_arms)*
                }
            }

            fn set_state(&mut self, state: homie5::HomieDeviceStatus) {
                match self {
                    #(#set_state_arms)*
                }
            }
        }

        impl hc_homie5::HomieDevice for #enum_name {
            type ResultError = #error_ty;

            fn publish_property_values(
                &mut self,
            ) -> impl std::future::Future<Output = Result<(), Self::ResultError>> + Send {
                async move {
                    match self {
                        #(#publish_prop_values_arms)*
                    }
                }
            }

            fn handle_set_command(
                &mut self,
                property: &homie5::PropertyRef,
                set_value: &str,
            ) -> impl std::future::Future<Output = Result<(), Self::ResultError>> + Send {
                async move {
                    match self {
                        #(#handle_set_command_arms)*
                    }
                }
            }

            fn publish_meta(
                &mut self,
            ) -> impl std::future::Future<Output = Result<(), Self::ResultError>> + Send {
                async move {
                    match self {
                        #(#publish_meta_arms)*
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}
