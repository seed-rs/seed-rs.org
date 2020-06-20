# How to generate the images

The only fully manually drawn image is `seed_logo.svg`, other images are mostly
generated with a sequence of automatic or manual steps:

1. The first step is to generate the small png logo.
   Using [Inkscape](https://inkscape.org) this can be done at the command line:

   ```bash
   inkscape --without-gui --file=seed_logo.svg --export-png=seed_logo_120.png --export-width=120
   ```

2. Export the capital _S_ in a new file.
   Using Inkscape again, is possible to export a single drawing item, by `id` using the command line:

   ```bash
   inkscape --without-gui --file=seed_logo.svg --export-id=S --export-id-only --vacuum-defs --export-plain-svg=seed_logo_square.svg
   ```

3. Make the logo square for real (there is no command line option to change the sheet size)

4. Once we have a `seed_logo_square.svg`, we can generate the 512x512 png image.
   Using the command line:

   ```bash
   inkscape --without-gui --file=seed_logo_square.svg --export-width=512 --export-height=512 --export-png=seed_logo_square_512.png
   ```

5. Reduce the generated png sizes (optional).
   On the command line, with `pngcrush`:

   ```bash
   pngcrush -ow seed_logo_120.png
   pngcrush -ow seed_logo_square_512.png
   ```

6. Reduce the size of the svg files for publishing, generating: `seed_logo.min.svg` and `seed_logo_square.min.svg`.
   On the command line this can be done using the `svgcleaner` utility (this is in Rust, cargo installable).

   ```bash
   svgcleaner --coordinates-precision 2 --properties-precision 2 --transforms-precision 2 --paths-coordinates-precision 2 seed_logo.svg seed_logo.min.svg
   svgcleaner --coordinates-precision 2 --properties-precision 2 --transforms-precision 2 --paths-coordinates-precision 2 seed_logo_square.svg seed_logo_square.min.svg
   ```

7. Save a modified copy of `seed_logo_square.svg`, changing the `viewBox` to `"0 0 16 16"`, to follow the
   [Apple Safari Pinned Tab specification](https://developer.apple.com/library/archive/documentation/AppleApplications/Reference/SafariWebContent/pinnedTabs/pinnedTabs.html)
   to `../favicons/safari-pinned-tab.svg`

8. Minimize with `svgcleaner` the new file.
   Using the command line:

   ```bash
   svgcleaner --coordinates-precision 2 --properties-precision 2 --transforms-precision 2 --paths-coordinates-precision 2 seed_logo_square.svg ../favicons/safari-pinned-tab.svg
   ```

9. Generate the _favicons_.
   This can be done using https://realfavicongenerator.net, uploading the square svg logo.

10. Compress the downloaded png files (optional)

11. The uncompressed `seed_logo_square.svg` can be removed, we can commit the `seed_logo_square.min.svg` version only.
