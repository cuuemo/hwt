/*
 * func-name: sub_140006F48
 * func-address: 0x140006f48
 * callers: 0x1400012fc
 * callees: none
 */

__int64 __fastcall sub_140006F48(HKEY a1, const WCHAR *a2, const WCHAR *a3, BYTE *a4, int a5)
{
  unsigned int v5; // ebx
  LSTATUS v8; // eax
  DWORD cbData; // [rsp+30h] [rbp-18h] BYREF
  HKEY hKey; // [rsp+38h] [rbp-10h] BYREF
  DWORD Type; // [rsp+68h] [rbp+20h] BYREF

  v5 = 0;
  *(_WORD *)a4 = 0;
  hKey = 0;
  v8 = RegOpenKeyExW(a1, a2, 0, 1u, &hKey);
  if ( v8 == 5 )
    return 0;
  if ( !v8 && hKey )
  {
    Type = 0;
    cbData = 2 * a5;
    RegQueryValueExW(hKey, a3, 0, &Type, a4, &cbData);
    RegCloseKey(hKey);
    if ( Type == 4 )
      wsprintfW((LPWSTR)a4, L"%i", *(unsigned __int16 *)a4);
  }
  LOBYTE(v5) = *(_WORD *)a4 != 0;
  return v5;
}
