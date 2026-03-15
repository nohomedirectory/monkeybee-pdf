Seven rounds have produced a 3300+ line spec that is architecturally complete, operationally grounded, and implementation-ready. This round focuses on tightening: cross-reference consistency between sections, precision in edge case handling, and closing remaining micro-gaps where the spec assumes knowledge it hasn't stated.

## 1) Specify the `PdfDictionary` duplicate key handling across all modes

The spec says "duplicate keys: last-definition-wins in tolerant mode, diagnostic reported" (Part 3), but doesn't specify the behavior in strict and preserve modes. This affects parsing, round-trip, and Arlington validation.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ PDF object type specifics
 - **Dictionary:** Key-value maps where keys are names and values are any object type. Duplicate
-  keys are technically malformed; the engine uses last-definition-wins semantics in tolerant mode
-  and reports a diagnostic.
+  keys are technically malformed. Handling depends on the parse mode:
+  - **Tolerant mode:** Last-definition-wins. A diagnostic (`parse.object.duplicate_key`) is
+    emitted with both the kept and discarded values.
+  - **Strict mode:** Duplicate keys are a validation error. The parser still produces a result
+    (using last-definition-wins) but the diagnostic is Error severity, which will cause
+    Arlington validation to fail.
+  - **Preserve mode:** Both entries are retained in the raw syntax layer
+    (`monkeybee-syntax`), preserving their byte spans and ordering. The semantic layer
+    (`monkeybee-document`) applies last-definition-wins when resolving the dictionary. This
+    ensures that preserve-mode round-trip emits the same bytes even if duplicates exist.
```

## 2) Specify the inline image `EI` detection algorithm

The spec mentions the difficulty of finding `EI` (Part 3, content stream parsing) but the algorithm is vague. This is one of the most subtle parsing challenges in PDF.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Inline image operators
 the parser must track the expected data length
+
+**EI detection algorithm:**
+
+Finding the `EI` operator that terminates inline image data is non-trivial because the image
+data can contain the bytes `E`, `I` in sequence. The algorithm:
+
+1. **Compute expected data length** from the image parameters: `ceil(W × BPC × components / 8)
+   × H`, where W = width, H = height, BPC = bits per component, components = number of color
+   components. For filtered images, this is the decoded length; the encoded length may differ.
+
+2. **For unfiltered images:** Skip exactly the computed number of bytes after `ID` (plus the
+   mandatory single whitespace byte after `ID`). Verify that the bytes at that position are
+   `EI` preceded by whitespace. If so, accept.
+
+3. **For filtered images (encoded length unknown):** Scan forward from the `ID` data start,
+   looking for the pattern: whitespace + `E` + `I` + (whitespace or end-of-stream). At each
+   candidate position, verify:
+   a. The byte before `E` is whitespace (SP, LF, CR, TAB, NUL, FF)
+   b. The byte after `I` is whitespace or triggers the next operator parse
+   c. The data between `ID` and the candidate `EI` is a valid encoded stream (attempt a trial
+      decode — if decompression succeeds and produces a plausible image, accept this position)
+
+4. **Fallback:** If no valid `EI` is found within a bounded scan (default: 1 MiB of data after
+   `ID`), the inline image is abandoned. The interpreter emits an error diagnostic and attempts
+   to resynchronize by scanning for the next known operator keyword.
+
+5. **Edge case — empty inline images:** An inline image with W=0 or H=0 has zero data bytes.
+   The `EI` immediately follows the whitespace after `ID`.
```

## 3) Specify form field inheritance resolution

The spec lists inheritable field attributes but doesn't specify the resolution algorithm. Field inheritance works differently from page inheritance (fields can be deeply nested with intermediate partial definitions).

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ AcroForm contract
 Inheritable field attributes: `/FT`, `/V`, `/DV`, `/Ff`, `/DA`, `/Q`.
+
+**Field inheritance resolution algorithm:**
+
+Field attributes propagate from parent to child in the field hierarchy. Unlike page inheritance
+(where the first ancestor with the attribute provides it), field inheritance has additional
+complexities:
+
+1. **Walk from field to root:** For each inheritable attribute, walk up the field hierarchy
+   via `/Parent` references. The first ancestor that defines the attribute provides its value.
+
+2. **`/FT` (field type):** Required on every terminal field. If a terminal field lacks `/FT`,
+   inherit from the nearest ancestor. If no ancestor defines it, the field is malformed —
+   report a diagnostic and skip the field.
+
+3. **`/Ff` (field flags):** Flags are inherited as a complete bitmask, not merged. A child's
+   `/Ff` replaces (not ORs with) the parent's flags. If a child has no `/Ff`, use the parent's.
+
+4. **`/V` (value) and `/DV` (default value):** Inherited normally. A common pattern: the parent
+   field defines `/DV` and each child inherits it unless overridden.
+
+5. **`/DA` (default appearance):** The appearance string (e.g., "/Helv 12 Tf 0 g") is inherited
+   as a complete string. The AcroForm dictionary's `/DA` entry serves as the document-level
+   default, applied when no field in the hierarchy defines `/DA`.
+
+6. **Partial field names:** A field's full name is constructed by concatenating ancestor names
+   with `.` separators. For example, if a field named `zip` has a parent named `address` which
+   has a parent named `form`, the full field name is `form.address.zip`. This is critical for
+   form data import/export — the full name is the unique identifier.
+
+7. **Widget-field relationship:** A widget annotation is the visual representation of a field.
+   A field can have multiple widgets (e.g., a radio button group). Widgets inherit all field
+   attributes from their parent field but can override visual properties via `/MK`.
```

## 4) Specify the object number allocation strategy for new objects

The spec says "new objects are assigned object numbers from a monotonically increasing allocator" but doesn't specify the starting point or how to handle object number reuse in full-rewrite mode.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Mutation safety
 - New objects are assigned object numbers from a monotonically increasing allocator.
+  The allocator starts at `max_existing_object_number + 1` for the current snapshot.
+  In incremental-append mode, new object numbers must not collide with any existing object
+  (including objects in the free list). In full-rewrite mode, object numbers may be reassigned
+  (compacted) starting from 1, since the entire xref is rebuilt. However, compaction changes
+  all internal references and must update every indirect reference in the document. The
+  baseline v1 writer does not compact object numbers in full-rewrite mode — it preserves
+  existing numbers and adds new objects at higher numbers. Compaction is an explicit
+  optimization operation in `monkeybee-edit`.
```

## 5) Add the `/Metadata` stream preservation requirement for XMP round-trip

The spec mentions XMP metadata preservation in the metadata extraction section but doesn't specify how the writer preserves it. XMP streams have specific requirements (padding, packet boundaries) that make them different from other streams.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ XMP metadata
 the engine must preserve XMP metadata byte-perfectly during round-trip operations
+(XMP packets often contain padding whitespace that must be preserved).
+
+**XMP stream preservation rules:**
+
+1. **Byte-perfect preservation:** The XMP metadata stream must be preserved byte-for-byte during
+   round-trip operations unless the user explicitly modifies metadata. This includes the XML
+   declaration, processing instructions, padding whitespace, and the packet wrapper
+   (`<?xpacket begin="..." id="..."?>` ... `<?xpacket end="w"?>`).
+
+2. **Padding preservation:** XMP packets typically include trailing whitespace padding (often
+   2048+ bytes of spaces) to allow in-place metadata updates without rewriting the stream. The
+   writer must preserve this padding. In incremental-append mode, the XMP stream is not touched
+   unless metadata was modified.
+
+3. **XMP modification:** When the user modifies metadata (title, author, etc.), the engine
+   updates the XMP packet in-place if there is sufficient padding. If not, a new XMP stream is
+   generated with fresh padding. The new stream must maintain the packet wrapper and include at
+   least 2048 bytes of padding.
+
+4. **Info dictionary synchronization:** When XMP is modified, the engine should also update the
+   corresponding Info dictionary entries to maintain consistency. When Info dictionary entries are
+   modified, the engine should also update XMP. The engine logs a diagnostic when the two
+   sources are already inconsistent on input.
```

## 6) Specify the resource naming strategy for generated content

The spec says the content builder "automatically tracks which fonts, images, and ExtGState dictionaries are used and generates the appropriate resource dictionaries" but doesn't specify the naming convention for generated resource names.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Resource management
 The caller does not manually manage resource names.
+
+**Resource naming convention for generated content:**
+
+When the content builder assigns names to resources in the resource dictionary:
+
+- **Fonts:** `/F1`, `/F2`, `/F3`, ... (incrementing integer suffix). For pages that reference
+  existing resources (e.g., annotation flattening), the builder checks existing font names and
+  continues from the highest existing number to avoid collisions.
+- **Images:** `/Im1`, `/Im2`, `/Im3`, ...
+- **Form XObjects:** `/Fm1`, `/Fm2`, `/Fm3`, ...
+- **ExtGState:** `/GS1`, `/GS2`, `/GS3`, ...
+- **Color spaces:** `/CS1`, `/CS2`, `/CS3`, ...
+- **Patterns:** `/P1`, `/P2`, `/P3`, ...
+- **Shadings:** `/Sh1`, `/Sh2`, `/Sh3`, ...
+
+The naming convention follows the common PDF producer practice. Names must be unique within a
+single resource dictionary. The builder maintains a name-to-object map and deduplicates: if the
+same font/image/ExtGState is used multiple times on the same page, it gets one resource name.
```

## 7) Specify the page content stream concatenation contract

The spec says pages can have multiple content streams that are "logically concatenated" but the concatenation semantics have edge cases that affect correctness.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Page rendering specifics
 5. If the page has multiple content streams (an array of stream references), concatenate them
-   logically with a space separator.
+   logically with a space separator. Specifically:
+   - Decode each stream independently through its filter chain.
+   - Concatenate the decoded bytes with a single SPACE (0x20) byte between each stream.
+   - The space separator prevents token merging: without it, the last token of stream N and the
+     first token of stream N+1 could merge into a single malformed token.
+   - The graphics state is NOT reset between streams. A `q` in stream 1 can be matched by `Q`
+     in stream 2. Fonts, colors, and all other state persist across stream boundaries.
+   - The content stream array may contain null references (some producers leave gaps). Null
+     entries are skipped without inserting a separator.
+   - An empty content stream array means the page has no content (blank page). This is legal.
+   - A single stream reference (not in an array) is treated identically to a one-element array.
```

## 8) Add the object stream extraction contract

The spec mentions object streams (Part 3, Part 5) but doesn't specify the extraction algorithm. Object streams are one of the most common structures in modern PDFs and a common source of parser bugs.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Object streams
 - Object streams (PDF 1.5+): multiple objects packed into a single compressed stream object.
+
+**Object stream extraction contract:**
+
+An object stream is a stream object with `/Type /ObjStm`. It contains:
+- `/N`: the number of objects in the stream
+- `/First`: the byte offset within the decoded stream data where the first object begins
+  (everything before `/First` is the index)
+- The decoded stream data contains: first an index of N pairs of (object_number, byte_offset)
+  as space-separated integers, then the objects themselves starting at byte offset `/First`
+
+**Extraction algorithm:**
+1. Decode the stream through its filter chain (typically FlateDecode).
+2. Parse the first `/First` bytes as the index: read N pairs of (object_number, byte_offset).
+   Each byte_offset is relative to `/First` (not to the start of the stream data).
+3. For each object, seek to `/First` + byte_offset and parse the object value. Objects in
+   object streams do NOT have the `N G obj ... endobj` wrapper — they are bare values.
+4. Register each extracted object in the cross-reference with its object number and generation
+   number 0 (objects in object streams always have generation 0).
+
+**Edge cases:**
+- Index corruption: if the index cannot be parsed (wrong number of entries, non-numeric values),
+  the entire object stream is skipped and all its objects are treated as missing. Record
+  diagnostic `parse.objstream.corrupt_index`.
+- Offset out of range: if a byte_offset points beyond the stream data, that individual object
+  is skipped. Other objects in the same stream are still extracted.
+- Nested object streams: an object stream containing another object stream is malformed per
+  spec. Detect and report; do not attempt recursive extraction.
+- The xref stream itself cannot be in an object stream.
```

## 9) Add the incremental save byte-range accounting for digital signatures

The spec describes signature preservation conceptually but doesn't specify how the writer accounts for byte offsets during incremental save. When appending to a file, all previously-written byte offsets are fixed, but new offsets must be carefully computed.

```diff
--- a/SPEC.md
+++ b/SPEC.md
@@ Signature-safe write path
 The writer must track which objects were modified
+
+**Incremental save byte-range accounting:**
+
+When writing an incremental update to a file with existing digital signatures:
+
+1. **Determine the append point:** The new content starts at the current end-of-file offset.
+   All previous bytes (0 to EOF-1) are immutable. The writer must not modify or rewrite any
+   byte before the append point.
+
+2. **Write new/modified objects:** Each object is serialized and its byte offset (relative to
+   file start) is recorded for the new xref. The offset = append_point + bytes_written_so_far.
+
+3. **Write the new cross-reference:** The new xref section covers only the new/modified objects.
+   Each entry's offset is computed as described above. Objects not in the new xref retain their
+   original offsets from the previous xref chain.
+
+4. **Write the new trailer:** The trailer's `/Prev` entry points to the byte offset of the
+   previous xref section (which is before the append point and therefore immutable). The
+   `/Size` covers the full object number space.
+
+5. **Signature byte-range verification post-write:** After writing, the engine verifies that
+   all existing signature byte ranges (`/ByteRange` arrays) are entirely within the immutable
+   region (before the append point). If any byte range extends to or beyond the append point,
+   this is an error — it means the original file was malformed or truncated.
+
+6. **`startxref` at end:** The new `startxref` value points to the byte offset of the new
+   xref section (within the appended region).
+
+This accounting ensures that no existing byte is modified, all new byte offsets are correct,
+and the incremental update chain is structurally valid.
```
