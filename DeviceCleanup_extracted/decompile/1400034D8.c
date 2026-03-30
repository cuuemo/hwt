/*
 * func-name: sub_1400034D8
 * func-address: 0x1400034d8
 * callers: 0x140003544
 * callees: 0x140001854
 */

__int64 __fastcall sub_1400034D8(int a1)
{
  __int64 v2; // rax
  int lParam; // [rsp+20h] [rbp-58h] BYREF
  _DWORD lParam_4[21]; // [rsp+24h] [rbp-54h] BYREF

  sub_140001854(lParam_4, 0, 0x44u);
  lParam_4[0] = a1;
  lParam = 4;
  if ( (unsigned int)SendMessageW(hWnd, 0x104Bu, 0, (LPARAM)&lParam)
    && (v2 = *(_QWORD *)(*(_QWORD *)qword_140010780 + 8LL * lParam_4[9])) != 0 )
  {
    return *(_QWORD *)(v2 + 2384);
  }
  else
  {
    return 0;
  }
}
