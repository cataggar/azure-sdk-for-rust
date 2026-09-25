// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// Adapted from the MIT-licensed azure-sdk-for-zig TCGC component.

// Stub for is-unicode-supported.
//
// Used only by TypeSpec's dynamic-task progress reporter for picking
// Unicode glyphs. Inside the component we always pretend the terminal
// is ASCII-only.

export default function isUnicodeSupported() {
  return false;
}
