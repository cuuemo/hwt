/*
 * func-name: sub_14000242C
 * func-address: 0x14000242c
 * callers: 0x1400036d4, 0x140003cec
 * callees: none
 */

LRESULT __fastcall sub_14000242C(unsigned int a1)
{
  WCHAR v2[132]; // [rsp+20h] [rbp-108h] BYREF

  wsprintfW(v2, L" Non-present Devices: %i", a1);
  return SendMessageW(qword_140010748, 0x40Bu, 0, (LPARAM)v2);
}
