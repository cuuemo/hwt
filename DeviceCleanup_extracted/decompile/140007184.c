/*
 * func-name: sub_140007184
 * func-address: 0x140007184
 * callers: 0x140006850
 * callees: none
 */

__int64 __fastcall sub_140007184(HKEY hKey, LPCWSTR lpSubKey, LPCWSTR lpValueName)
{
  LSTATUS v6; // eax
  unsigned int v7; // ebx
  LSTATUS v8; // edi
  HKEY hKeya; // [rsp+58h] [rbp+20h] BYREF

  SetLastError(0);
  v6 = RegOpenKeyExW(hKey, lpSubKey, 0, 2u, &hKeya);
  v7 = 0;
  v8 = v6;
  if ( !v6 )
  {
    v8 = RegDeleteValueW(hKeya, lpValueName);
    RegCloseKey(hKeya);
  }
  SetLastError(v8);
  LOBYTE(v7) = v8 == 0;
  return v7;
}
