# vfbreader

`vfbreader` is a parser for VFB files, the file format of FontLab Studio up until
version 5. It includes a command line utility, `vfbreader`, that will read the entries
from a VFB file and serialize them to JSON.

There is a more complete implementation in Python, [vfbLib](https://github.com/lucasfonts/vfbLib)
which also has write support for VFB files.

## Command Line Usage

```sh
vfbreader MyFile.vfb > MyFile.vfb.json
```

## License

`vfbreader` is licensed under the GNU General Public License or the GNU Lesser
General Public License.
