/*
 * func-name: sub_1400023E4
 * func-address: 0x1400023e4
 * callers: 0x1400012fc
 * callees: none
 */

LRESULT __fastcall sub_1400023E4(unsigned int a1, unsigned int a2)
{
  LRESULT result; // rax
  WCHAR v3[132]; // [rsp+20h] [rbp-108h] BYREF

  if ( (a1 & 0x3F) == 0 )
  {
    wsprintfW(v3, L" Enum Device %i of %i", a1, a2);
    return SendMessageW(qword_140010748, 0x40Bu, 0, (LPARAM)v3);
  }
  return result;
}
