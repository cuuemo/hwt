/*
 * func-name: sub_140007000
 * func-address: 0x140007000
 * callers: 0x140006850
 * callees: none
 */

__int64 __fastcall sub_140007000(HKEY a1, const WCHAR *a2, const WCHAR *a3, BYTE *a4)
{
  unsigned int v4; // ebx
  LSTATUS v7; // edi
  DWORD Type; // [rsp+30h] [rbp-18h] BYREF
  DWORD cbData; // [rsp+34h] [rbp-14h] BYREF
  HKEY hKey; // [rsp+38h] [rbp-10h] BYREF

  v4 = 0;
  hKey = 0;
  v7 = RegOpenKeyExW(a1, a2, 0, 1u, &hKey);
  if ( !v7 && hKey )
  {
    Type = 0;
    cbData = 4;
    v7 = RegQueryValueExW(hKey, a3, 0, &Type, a4, &cbData);
    if ( !v7 && Type == 1 )
      *(_DWORD *)a4 = StrToIntA((PCSTR)a4);
    RegCloseKey(hKey);
  }
  SetLastError(v7);
  LOBYTE(v4) = v7 == 0;
  return v4;
}
