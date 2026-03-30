/*
 * func-name: sub_140008274
 * func-address: 0x140008274
 * callers: 0x140001058
 * callees: none
 */

__int64 __fastcall sub_140008274(HKEY a1, const WCHAR *a2, const WCHAR *a3, const BYTE *a4, DWORD cbData)
{
  unsigned int v5; // ebx
  LSTATUS v8; // edi
  DWORD v10; // [rsp+50h] [rbp-18h] BYREF
  HKEY hKey; // [rsp+58h] [rbp-10h] BYREF

  v5 = 0;
  hKey = 0;
  v10 = 0;
  v8 = RegCreateKeyExW(a1, a2, 0, 0, 0, 6u, 0, &hKey, &v10);
  if ( !v8 && hKey )
  {
    v8 = RegSetValueExW(hKey, a3, 0, 3u, a4, cbData);
    RegCloseKey(hKey);
  }
  LOBYTE(v5) = v8 == 0;
  return v5;
}
