# Long Path Problem - Windows

Check where are the limits of long path on Windows.

## Usage

Build sample application:

```powershell
cd hello
cargo build --release
```


Run the tester
```powershell
cd long-path-validator
cargo run
```


Check Windows registry Long Path settings:
```powershell
Get-ItemProperty -Path "HKLM:\SYSTEM\CurrentControlSet\Control\FileSystem" -Name LongPathsEnabled
```


Check presence of manifest:
```powershell
mt.exe "-inputresource:.\target\debug\long-path-validator-manifest.exe;#1" -out:extracted_manifest.xml
```

Trace problems
```powershell
sxstrace.exe trace -logfile:sxs_trace.etl
```

Parse the trace
```powershell
sxstrace.exe parse -logfile:sxs_trace.etl -outfile:sxs_trace.txt
```

