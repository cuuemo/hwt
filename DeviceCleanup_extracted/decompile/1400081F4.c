/*
 * func-name: sub_1400081F4
 * func-address: 0x1400081f4
 * callers: 0x140001130
 * callees: none
 */

__int64 __fastcall sub_1400081F4(HKEY a1, const WCHAR *a2, const WCHAR *a3, BYTE *a4, DWORD cbData)
{
  DWORD Type; // [rsp+30h] [rbp-18h] BYREF
  HKEY hKey; // [rsp+38h] [rbp-10h] BYREF

  hKey = 0;
  if ( RegOpenKeyExW(a1, a2, 0, 1u, &hKey) || !hKey )
    return 0;
  Type = 0;
  RegQueryValueExW(hKey, a3, 0, &Type, a4, &cbData);
  RegCloseKey(hKey);
  return cbData;
}
