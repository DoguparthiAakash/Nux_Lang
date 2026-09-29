call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
rc.exe resources.rc
cl.exe /O2 /EHsc /MD main.cpp resources.res user32.lib gdi32.lib shell32.lib comctl32.lib /FeNuxSetup.exe
