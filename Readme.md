# gdal

A fork of [https://github.com/georust/gdal]() for our usage.

## Building
### On Windows

Note that the build script automatically downloads a prebuilt zip of gdal. 
If you don't want it to do so, set the `GDAL_NO_DOWNLOAD` environment variable, and follow the other instructions.

### On not-Windows
Other platforms do not have prebuilt binaries, so set `GDAL_PATH` to the path of a gdal static library.
