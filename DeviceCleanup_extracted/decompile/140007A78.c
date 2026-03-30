/*
 * func-name: sub_140007A78
 * func-address: 0x140007a78
 * callers: 0x14000453c, 0x1400047fc
 * callees: 0x140001854
 */

LRESULT __fastcall sub_140007A78(HWND a1, int a2, int a3)
{
  HWND v5; // rdi
  int v6; // ebx
  LRESULT result; // rax
  unsigned int v8; // eax
  int lParam; // [rsp+20h] [rbp-48h] BYREF
  LPARAM lParam_4; // [rsp+24h] [rbp-44h] BYREF
  unsigned int v11; // [rsp+3Ch] [rbp-2Ch]

  v5 = (HWND)SendMessageW(a1, 0x101Fu, 0, 0);
  sub_140001854(&lParam_4, 0, 0x3Cu);
  lParam = 4;
  v6 = 0;
  result = SendMessageW(v5, 0x1200u, 0, 0);
  if ( (int)result > 0 )
  {
    do
    {
      if ( (unsigned int)SendMessageW(v5, 0x120Bu, v6, (LPARAM)&lParam) )
      {
        if ( v6 == a2 )
        {
          if ( a3 )
            v8 = v11 & 0xFFFFF9FF | 0x400;
          else
            v8 = v11 & 0xFFFFF9FF | 0x200;
          v11 = v8;
        }
        else
        {
          v11 &= 0xFFFFF9FF;
        }
        SendMessageW(v5, 0x120Cu, v6, (LPARAM)&lParam);
      }
      ++v6;
      result = SendMessageW(v5, 0x1200u, 0, 0);
    }
    while ( v6 < (int)result );
  }
  return result;
}
