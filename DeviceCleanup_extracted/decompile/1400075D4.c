/*
 * func-name: sub_1400075D4
 * func-address: 0x1400075d4
 * callers: 0x1400047fc, 0x140006850
 * callees: 0x140001854
 */

_BOOL8 sub_1400075D4()
{
  int v0; // r11d
  struct _OSVERSIONINFOW VersionInformation; // [rsp+20h] [rbp-128h] BYREF

  sub_140001854(&VersionInformation, 0, 0x11Cu);
  VersionInformation.dwOSVersionInfoSize = 284;
  GetVersionExW(&VersionInformation);
  v0 = dword_14001066C + (dword_140010668 << 8);
  return v0 >= 1281 || v0 == 1168;
}
