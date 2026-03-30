/*
 * func-name: sub_1400070B8
 * func-address: 0x1400070b8
 * callers: 0x140006850
 * callees: 0x14000179c, 0x1400018e8
 */

__int64 __fastcall sub_1400070B8(HKEY a1, const WCHAR *a2, const WCHAR *a3, int a4)
{
  unsigned int v4; // ebx
  LSTATUS v6; // eax
  LSTATUS v7; // edi
  DWORD v9; // [rsp+50h] [rbp-18h] BYREF
  HKEY hKey; // [rsp+58h] [rbp-10h] BYREF
  int Data; // [rsp+88h] [rbp+20h] BYREF

  Data = a4;
  v4 = 0;
  hKey = 0;
  v9 = 0;
  v6 = RegCreateKeyExW(a1, a2, 0, 0, 0, 6u, 0, &hKey, &v9);
  v7 = v6;
  if ( v6 )
  {
    sub_1400018E8(v6);
    sub_14000179C();
    return 0;
  }
  else
  {
    if ( hKey )
    {
      v7 = RegSetValueExW(hKey, a3, 0, 4u, (const BYTE *)&Data, 4u);
      RegCloseKey(hKey);
    }
    SetLastError(v7);
    LOBYTE(v4) = v7 == 0;
    return v4;
  }
}
