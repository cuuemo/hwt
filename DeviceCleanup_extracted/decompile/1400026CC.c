/*
 * func-name: sub_1400026CC
 * func-address: 0x1400026cc
 * callers: 0x140004704
 * callees: 0x140001854, 0x140007560
 */

BOOL __fastcall sub_1400026CC(HWND hWnd, __int64 a2, LONG a3, LONG a4)
{
  LONG v4; // ebx
  LONG v5; // edi
  BOOL result; // eax
  struct tagPOINT Point; // [rsp+40h] [rbp-38h] BYREF
  int v9; // [rsp+48h] [rbp-30h] BYREF
  int v10; // [rsp+4Ch] [rbp-2Ch]
  int v11; // [rsp+50h] [rbp-28h]
  int v12; // [rsp+54h] [rbp-24h]
  struct tagPOINT v13; // [rsp+58h] [rbp-20h] BYREF
  int v14; // [rsp+68h] [rbp-10h]

  v4 = a4;
  v5 = a3;
  if ( a3 == -1 && a4 == -1 )
  {
    v9 = 2;
    SendMessageW(::hWnd, 0x100Eu, dword_14000F45C, (LPARAM)&v9);
    sub_140007560(::hWnd, &v9);
    v5 = v9 + (v11 - v9) / 2;
    v4 = v10 + (v12 - v10) / 2;
  }
  Point.x = v5;
  Point.y = v4;
  result = ScreenToClient(::hWnd, &Point);
  if ( Point.y > dword_14000FFF8 )
  {
    sub_140001854(&v13.y, 0, 0x10u);
    v13.x = v5;
    v13.y = v4;
    ScreenToClient(::hWnd, &v13);
    dword_14000F45C = SendMessageW(::hWnd, 0x1039u, 0, (LPARAM)&v13);
    dword_14000F460 = v14;
    return TrackPopupMenu(hMenu, 2u, v5, v4, 0, hWnd, 0);
  }
  return result;
}
