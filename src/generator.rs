use std::borrow::Cow;

use openapiv3::{OpenAPI, Schema};

use crate::schema;

#[derive(Debug)]
pub struct Generator {
    default_options: GeneratorOptions,
    openapi: OpenAPI,
}

#[derive(Debug, Default, Clone)]
pub struct GeneratorOptions {
    /// Determines whether fields marked as `readOnly` should be included in the generated type definition.
    ///
    /// If `true`, fields with the `readOnly` attribute will be excluded from the type definition.
    /// This can be useful for generating types intended for input or update operations where read-only fields should not be set.
    ///
    /// If `false`, fields with the `readOnly` attribute will be included in the type definition.
    /// This is useful for generating types that represent the full structure, including fields that are read-only and should not be modified.
    pub skip_read_only: bool,
}

#[derive(Debug)]
pub struct PartialGeneration<'a> {
    pub typescript: Cow<'a, str>,
    pub references: Vec<Cow<'a, str>>,
    pub read_only: bool,
}

impl Generator {
    /// Creates a new instance of `Generator`.
    ///
    /// # Arguments
    ///
    /// * `openapi` - An instance of the `OpenAPI` structure representing the OpenAPI specification.
    /// * `default_options` - An instance of `GeneratorOptions` to configure the generator's behavior.
    ///
    /// # Returns
    ///
    /// A new `Generator` instance initialized with the provided OpenAPI specification and generator options.
    ///
    /// # Examples
    ///
    /// ```
    /// use openapiv3::OpenAPI;
    /// use openapiv3_to_typescript::{Generator, GeneratorOptions};
    ///
    /// let openapi = OpenAPI::default();
    /// let options = GeneratorOptions::default();
    /// let generator = Generator::new(openapi, options);
    /// ```
    pub fn new(openapi: OpenAPI, default_options: GeneratorOptions) -> Self {
        Self {
            openapi,
            default_options,
        }
    }

    /// Generates a TypeScript type definition from an OpenAPI schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - A reference to the `Schema` object from the OpenAPI specification that you want to generate the TypeScript type for.
    /// * `options` - An optional instance of `GeneratorOptions` to override the default options for this generation.
    ///
    /// # Returns
    ///
    /// A `PartialGeneration` instance containing the generated TypeScript type definition as a string
    /// and a vector of references that were included in the generation process.
    ///
    /// # Examples
    ///
    /// ```
    /// use openapiv3::{OpenAPI, Schema};
    /// use openapiv3_to_typescript::{Generator, GeneratorOptions, PartialGeneration};
    ///
    /// let openapi = OpenAPI::default();
    /// let options = GeneratorOptions { skip_read_only: true };
    /// let generator = Generator::new(openapi, options);
    /// let schema = Schema::new_number();
    /// let partial_generation: PartialGeneration = generator.generate_schema(&schema, None);
    ///
    /// println!("{:?}", partial_generation);
    /// ```
    pub fn generate_schema<'a>(
        &'a self,
        schema: &'a Schema,
        options: Option<GeneratorOptions>,
    ) -> PartialGeneration<'a> {
        let cow = match options {
            None => Cow::Borrowed(&self.default_options),
            Some(options) => Cow::Owned(options),
        };

        schema::generate_schema(schema, cow)
    }

    pub fn openapi(&self) -> &OpenAPI {
        &self.openapi
    }
}

impl<'a> PartialGeneration<'a> {
    pub fn typescript_mut(&mut self) -> &mut Cow<'a, str> {
        &mut self.typescript
    }
}
