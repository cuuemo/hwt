/*
 * func-name: sub_140008158
 * func-address: 0x140008158
 * callers: 0x140001130
 * callees: none
 */

__int64 __fastcall sub_140008158(HKEY a1, const WCHAR *a2, const WCHAR *a3, DWORD *a4)
{
  unsigned int v4; // ebx
  LSTATUS v7; // edi
  bool v8; // zf
  DWORD Type; // [rsp+30h] [rbp-18h] BYREF
  HKEY hKey; // [rsp+38h] [rbp-10h] BYREF

  v4 = 0;
  hKey = 0;
  v7 = RegOpenKeyExW(a1, a2, 0, 1u, &hKey);
  if ( v7 )
    goto LABEL_4;
  if ( !hKey )
  {
LABEL_5:
    v8 = v7 == 0;
    goto LABEL_6;
  }
  Type = 0;
  v7 = RegQueryValueExW(hKey, a3, 0, &Type, 0, a4);
  RegCloseKey(hKey);
  v8 = v7 == 0;
  if ( v7 )
  {
LABEL_4:
    SetLastError(v7);
    goto LABEL_5;
  }
LABEL_6:
  LOBYTE(v4) = v8;
  return v4;
}
