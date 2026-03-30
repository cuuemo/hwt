/*
 * func-name: sub_140007638
 * func-address: 0x140007638
 * callers: 0x140007738
 * callees: none
 */

__int64 __fastcall sub_140007638(HKEY a1, const WCHAR *a2, struct _FILETIME *a3)
{
  HKEY hKey; // [rsp+60h] [rbp-18h] BYREF
  DWORD cSubKeys; // [rsp+98h] [rbp+20h] BYREF

  if ( RegOpenKeyExW(a1, a2, 0, 0x20019u, &hKey) )
    return 2;
  cSubKeys = 0;
  RegQueryInfoKeyW(hKey, 0, 0, 0, &cSubKeys, 0, 0, 0, 0, 0, 0, a3);
  RegCloseKey(hKey);
  return 0;
}
