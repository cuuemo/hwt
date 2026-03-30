/*
 * func-name: sub_140007428
 * func-address: 0x140007428
 * callers: 0x140002a74, 0x140006378, 0x140006d80
 * callees: none
 */

HMODULE __fastcall sub_140007428(LPCSTR lpString2)
{
  CHAR Buffer[280]; // [rsp+20h] [rbp-118h] BYREF

  GetSystemDirectoryA(Buffer, 0x104u);
  lstrcatA(Buffer, "\\");
  lstrcatA(Buffer, lpString2);
  return LoadLibraryA(Buffer);
}
