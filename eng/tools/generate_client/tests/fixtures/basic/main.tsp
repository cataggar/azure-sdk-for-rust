// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

import "@typespec/http";
import "@typespec/rest";

using TypeSpec.Http;

@service(#{ title: "Basic" })
namespace Basic;

model Widget {
  name: string;
}

@route("/widgets")
interface Widgets {
  @get getWidget(@path widget_id: string): Widget;
}
