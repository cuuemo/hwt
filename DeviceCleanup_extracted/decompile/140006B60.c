/*
 * func-name: sub_140006B60
 * func-address: 0x140006b60
 * callers: 0x1400052dc
 * callees: 0x140001854
 */

_BOOL8 __fastcall sub_140006B60(WCHAR *a1)
{
  signed int dwMajorVersion; // r11d
  signed int dwMinorVersion; // r10d
  int v4; // edi
  int v5; // ecx
  BOOL v6; // eax
  BOOL v7; // eax
  _BOOL8 result; // rax
  _OSVERSIONINFOW VersionInformation; // [rsp+20h] [rbp-128h] BYREF
  unsigned __int16 v10; // [rsp+134h] [rbp-14h]
  unsigned __int16 v11; // [rsp+138h] [rbp-10h]
  unsigned __int8 v12; // [rsp+13Ah] [rbp-Eh]

  sub_140001854(&VersionInformation, 0, 0x11Cu);
  VersionInformation.dwOSVersionInfoSize = 284;
  GetVersionExW(&VersionInformation);
  sub_140001854(a1, 0, 0x74u);
  dwMajorVersion = VersionInformation.dwMajorVersion;
  dwMinorVersion = VersionInformation.dwMinorVersion;
  *((_DWORD *)a1 + 12) = VersionInformation.dwBuildNumber;
  *((_DWORD *)a1 + 13) = v10;
  *((_DWORD *)a1 + 14) = v12;
  *((_DWORD *)a1 + 15) = v11;
  *((_DWORD *)a1 + 11) = dwMinorVersion;
  *((_DWORD *)a1 + 10) = dwMajorVersion;
  v4 = 1;
  *((_DWORD *)a1 + 16) = dwMinorVersion % 10
                       + 16 * (dwMinorVersion / 10 + 16 * (dwMajorVersion % 10 + 16 * (dwMajorVersion / 10)));
  if ( dwMajorVersion >= 5 )
  {
    *((_DWORD *)a1 + 18) = 1;
    lstrcpyW(a1 + 42, L"x64");
  }
  v5 = *((_DWORD *)a1 + 16);
  *(_DWORD *)a1 = v5 == 1280;
  *((_DWORD *)a1 + 1) = v5 == 1281;
  v6 = v5 == 1282 && *((_DWORD *)a1 + 14) == 1 && (*((_DWORD *)a1 + 19) || *((_DWORD *)a1 + 18));
  *((_DWORD *)a1 + 2) = v6;
  v7 = v5 == 1282 && *((_DWORD *)a1 + 14) != 1;
  *((_DWORD *)a1 + 3) = v7;
  result = v5 == 1536;
  *((_DWORD *)a1 + 4) = result;
  if ( v5 < 1537 || v5 >= 1792 )
    v4 = 0;
  *((_DWORD *)a1 + 5) = v4;
  return result;
}
