// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import "@typespec/http";
import "@typespec/rest";

using TypeSpec.Http;

@service(#{ title: "Smoke" })
namespace Smoke;

model Widget {
  name: string;
}

@route("/widgets")
interface Widgets {
  @get list(): Widget[];
}
