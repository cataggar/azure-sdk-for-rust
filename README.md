# Azure SDK for Rust

## Legacy OpenAPI Services Crates

The crates published in this `legacy` branch are unofficial & unsupported by Azure Support or the Azure SDK team. They have been built primarily by volunteers on their own time. For more information, see the [project history](https://github.com/Azure/azure-sdk-for-rust/blob/legacy/HISTORY.md). These Azure service crates are generated from the OpenAPI specficications in [Azure REST API Specifications](https://github.com/Azure/azure-rest-api-specs). The official SDKs use the newer TypeSpec specifications.

> Microsoft is developing the official Azure SDK for Rust crates and has no plans to update these unofficial crates.
> In the future we may release official versions that may have a different package names.
> If releasing an official version of this crate is important to you [let us know](https://github.com/Azure/azure-sdk-for-rust/issues/new/choose).
>
> Source for this crate can now be found in <https://github.com/Azure/azure-sdk-for-rust/tree/legacy>.
> To monitor for an official, supported version of this crate, see <https://aka.ms/azsdk/releases>.

## Status

🚨 WARNING 🚨: This project is no longer under active development. Work on officially supported crates is being done in the `main` branch. Pull requests to `legacy` may be considered but are not guaranteed to be merged. Instead, you might consider forking this repository, creating a topic branch you'd need to submit a pull request anyway, and [override dependencies](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html) in your `Cargo.toml` file(s) as necessary.

This project is the successor to the `azure_sdk*` crates from [MindFlavor/AzureSDKForRust](https://github.com/MindFlavor/AzureSDKForRust). The crates have been renamed, so those older crates should be considered fully deprecated. See [history](HISTORY.md) for more details.

## Contributing

This project welcomes contributions and suggestions.  Most contributions require you to agree to a
Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us
the rights to use your contribution. For details, visit https://cla.opensource.microsoft.com.

When you submit a pull request, a CLA bot will automatically determine whether you need to provide
a CLA and decorate the PR appropriately (e.g., status check, comment). Simply follow the instructions
provided by the bot. You will only need to do this once across all repos using our CLA.

This project has adopted the [Microsoft Open Source Code of Conduct](https://opensource.microsoft.com/codeofconduct/).
For more information see the [Code of Conduct FAQ](https://opensource.microsoft.com/codeofconduct/faq/) or
contact [opencode@microsoft.com](mailto:opencode@microsoft.com) with any additional questions or comments.
