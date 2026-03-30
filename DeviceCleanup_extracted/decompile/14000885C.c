/*
 * func-name: sub_14000885C
 * func-address: 0x14000885c
 * callers: 0x1400052dc
 * callees: 0x140006ce0, 0x140007208, 0x140008718, 0x1400087e8, 0x140008820
 */

DWORD sub_14000885C()
{
  LPWSTR FileNameW; // rax
  BOOL v1; // ebx
  _FILETIME v2; // rax
  _FILETIME v3; // rbx
  WCHAR FileName[264]; // [rsp+40h] [rbp-848h] BYREF
  WCHAR WideCharStr[264]; // [rsp+250h] [rbp-638h] BYREF
  WCHAR Dst[264]; // [rsp+460h] [rbp-428h] BYREF
  WCHAR ReturnedString[268]; // [rsp+670h] [rbp-218h] BYREF

  MultiByteToWideChar(0, 0, MultiByteStr, -1, WideCharStr, 260);
  ExpandEnvironmentStringsW(L"%ALLUSERSPROFILE%", Dst, 0x104u);
  sub_140006CE0(Dst, 0x5Cu);
  lstrcatW(Dst, L"Uwe Sieber");
  FileNameW = PathFindFileNameW(WideCharStr);
  wsprintfW(FileName, L"%s\\%s", Dst, FileNameW);
  v1 = sub_140008820(FileName);
  if ( v1
    && GetPrivateProfileStringW(L"Settings", L"NoIniWrite", &Default, ReturnedString, 0x104u, FileName)
    && !lstrcmpW(ReturnedString, &pszPath) )
  {
    return (unsigned int)lstrcpyW(WideCharStr, FileName);
  }
  v2.dwLowDateTime = sub_140008718(WideCharStr);
  if ( v2.dwLowDateTime )
  {
    if ( v1 )
    {
      v3 = sub_140007208(FileName);
      v2 = sub_140007208(WideCharStr);
      if ( *(_QWORD *)&v3 > *(unsigned __int64 *)&v2 )
        v2.dwLowDateTime = CopyFileW(FileName, WideCharStr, 0);
    }
  }
  else
  {
    v2.dwLowDateTime = sub_1400087E8(WideCharStr);
    if ( !v2.dwLowDateTime )
    {
      if ( !sub_140008820(Dst) )
        CreateDirectoryW(Dst, 0);
      if ( sub_140008820(WideCharStr) && !sub_140008820(FileName) )
        CopyFileW(WideCharStr, FileName, 0);
      WideCharToMultiByte(0, 0, FileName, -1, MultiByteStr, 260, 0, 0);
      v2.dwLowDateTime = WritePrivateProfileStringW(L"Settings", L"NoIniWrite", &pszPath, FileName);
    }
  }
  return v2.dwLowDateTime;
}
