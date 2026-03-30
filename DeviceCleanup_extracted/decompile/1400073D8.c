/*
 * func-name: sub_1400073D8
 * func-address: 0x1400073d8
 * callers: 0x140002e08, 0x140006850, 0x140007478
 * callees: none
 */

HMODULE __fastcall sub_1400073D8(LPCWSTR lpString2)
{
  WCHAR Buffer[268]; // [rsp+20h] [rbp-218h] BYREF

  GetSystemDirectoryW(Buffer, 0x104u);
  lstrcatW(Buffer, L"\\");
  lstrcatW(Buffer, lpString2);
  return LoadLibraryW(Buffer);
}
