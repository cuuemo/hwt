/*
 * func-name: sub_140003438
 * func-address: 0x140003438
 * callers: 0x1400036d4
 * callees: none
 */

LRESULT __fastcall sub_140003438(HWND hWnd, int a2)
{
  WPARAM v2; // rsi
  signed int v4; // ebx
  int v5; // eax

  v2 = a2;
  v4 = SendMessageW(hWnd, 0x1028u, 0, 0) + a2;
  if ( v4 >= (int)(SendMessageW(hWnd, 0x1004u, 0, 0) - 1) )
    v5 = SendMessageW(hWnd, 0x1004u, 0, 0) - 1;
  else
    v5 = v2 + SendMessageW(hWnd, 0x1028u, 0, 0);
  SendMessageW(hWnd, 0x1013u, v5, 0);
  return SendMessageW(hWnd, 0x1013u, v2, 0);
}
