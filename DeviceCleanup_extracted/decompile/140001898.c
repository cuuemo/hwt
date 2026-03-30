/*
 * func-name: sub_140001898
 * func-address: 0x140001898
 * callers: 0x140001130, 0x1400012fc, 0x140002e90, 0x1400036d4, 0x140005518, 0x140006264, 0x140008320, 0x140009100
 * callees: none
 */

LPVOID __fastcall sub_140001898(SIZE_T dwBytes)
{
  SIZE_T i; // r8
  LPVOID result; // rax

  for ( i = dwBytes; ; i = dwBytes )
  {
    result = HeapAlloc(hHeap, 8u, i);
    if ( result )
      break;
    Sleep(0x1F4u);
  }
  return result;
}
