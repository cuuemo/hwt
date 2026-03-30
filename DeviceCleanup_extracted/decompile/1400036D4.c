/*
 * func-name: sub_1400036D4
 * func-address: 0x1400036d4
 * callers: 0x140003c04, 0x140003cec
 * callees: 0x14000124c, 0x1400012fc, 0x140001854, 0x140001898, 0x14000242c, 0x14000246c, 0x140002d38, 0x140003438, 0x140007268, 0x1400078b4
 */

void sub_1400036D4()
{
  HWND DlgItem; // rax
  int v1; // ebp
  __int64 v2; // rax
  LPVOID v3; // rax
  __int64 v4; // rcx
  int v5; // edi
  __int64 v6; // rsi
  __int64 v7; // rbx
  int v8; // eax
  __int64 v9; // rcx
  __int64 v10; // rax
  unsigned int v11; // eax
  unsigned int v12; // eax
  int v13; // r11d
  int lParam; // [rsp+20h] [rbp-B8h] BYREF
  int lParam_4; // [rsp+24h] [rbp-B4h] BYREF
  int v16; // [rsp+28h] [rbp-B0h]
  int v17; // [rsp+2Ch] [rbp-ACh]
  int v18; // [rsp+30h] [rbp-A8h]
  WCHAR *v19; // [rsp+38h] [rbp-A0h]
  int v20; // [rsp+44h] [rbp-94h]
  __int64 v21; // [rsp+48h] [rbp-90h]
  WCHAR v22[32]; // [rsp+70h] [rbp-68h] BYREF
  int ImageIndex; // [rsp+E0h] [rbp+8h] BYREF

  if ( !dword_14001078C )
  {
    sub_140002D38((LPCWSTR)0x7F02, 0);
    SendMessageW(hWnd, 0xBu, 0, 0);
    DlgItem = GetDlgItem(hDlg, 1005);
    SetWindowTextW(DlgItem, &Default);
    v1 = SendMessageW(hWnd, 0x1027u, 0, 0);
    SendMessageW(hWnd, 0x1009u, 0, 0);
    v2 = qword_140010780;
    if ( !qword_140010780 )
    {
      v3 = sub_140001898(0x18u);
      if ( v3 )
        v2 = sub_14000124C((__int64)v3);
      else
        v2 = 0;
      qword_140010780 = v2;
    }
    sub_1400012FC(v2);
    SendMessageW(qword_140010748, 0x40Bu, 0, (LPARAM)L"Fill ListView");
    v4 = qword_140010780;
    v5 = 0;
    if ( *(int *)(qword_140010780 + 8) > 0 )
    {
      v6 = 0;
      do
      {
        v7 = *(_QWORD *)(v6 + *(_QWORD *)v4);
        if ( *(_DWORD *)(v7 + 12) == 45 )
        {
          if ( (dword_1400107C0 || !sub_140007268((PCWSTR)(v7 + 16), L"ROOT\\"))
            && (dword_1400107C4 || !sub_140007268((PCWSTR)(v7 + 16), L"SWD\\"))
            && (dword_1400107C8 || !sub_140007268((PCWSTR)(v7 + 16), L"SW\\{")) )
          {
            ImageIndex = 0;
            if ( SetupDiGetClassImageIndex(&ClassImageListData, (const GUID *)(v7 + 2364), &ImageIndex) )
            {
              sub_140001854(&lParam_4, 0, 0x44u);
              v20 = ImageIndex;
              v19 = (WCHAR *)(v7 + 1084);
              lParam = 7;
              v21 = v5;
              v8 = SendMessageW(hWnd, 0x104Du, 0, (LPARAM)&lParam);
              v9 = *(_QWORD *)(v7 + 2384);
              lParam_4 = v8;
              if ( v9 > 0 )
              {
                v10 = sub_1400078B4();
                wsprintfW(v22, L"%s", v10);
                v19 = v22;
                lParam = 9;
                v16 = 1;
                SendMessageW(hWnd, 0x104Cu, 0, (LPARAM)&lParam);
              }
              v19 = (WCHAR *)(v7 + 2108);
              lParam = 9;
              v16 = 2;
              SendMessageW(hWnd, 0x104Cu, 0, (LPARAM)&lParam);
              if ( dword_140010798 )
              {
                v16 = dword_14000F464;
                lParam = 9;
                v19 = (WCHAR *)(v7 + 1852);
                SendMessageW(hWnd, 0x104Cu, 0, (LPARAM)&lParam);
              }
              if ( dword_14001079C )
              {
                v16 = dword_14000F468;
                lParam = 9;
                v19 = (WCHAR *)(v7 + 1596);
                SendMessageW(hWnd, 0x104Cu, 0, (LPARAM)&lParam);
              }
              if ( dword_1400107A0 )
              {
                v16 = dword_14000F46C;
                lParam = 9;
                v19 = (WCHAR *)(v7 + 536);
                SendMessageW(hWnd, 0x104Cu, 0, (LPARAM)&lParam);
              }
              if ( dword_140010794 )
              {
                v16 = dword_14000F470;
                lParam = 9;
                v19 = (WCHAR *)(v7 + 16);
                SendMessageW(hWnd, 0x104Cu, 0, (LPARAM)&lParam);
              }
            }
          }
          v4 = qword_140010780;
        }
        ++v5;
        v6 += 8;
      }
      while ( v5 < *(_DWORD *)(v4 + 8) );
    }
    v11 = SendMessageW(hWnd, 0x1004u, 0, 0);
    sub_14000242C(v11);
    v18 = 4;
    v17 = 4;
    SendMessageW(hWnd, 0x102Bu, 0xFFFFFFFFFFFFFFFFuLL, (LPARAM)&lParam);
    v12 = SendMessageW(hWnd, 0x1004u, 0, 0);
    sub_14000242C(v12);
    sub_14000246C();
    v13 = dword_14000F45C;
    if ( *(_DWORD *)(qword_140010780 + 8) < dword_14000F45C )
      v13 = 0;
    dword_14000F45C = v13;
    SendMessageW(hWnd, 0x1051u, dword_140010790, (LPARAM)sub_140003584);
    v18 = 3;
    v17 = 3;
    SendMessageW(hWnd, 0x102Bu, dword_14000F45C, (LPARAM)&lParam);
    SetFocus(hWnd);
    sub_140003438(hWnd, v1);
    SendMessageW(hWnd, 0xBu, 1u, 0);
    sub_140002D38((LPCWSTR)0x7F00, 1);
    UpdateWindow(hWnd);
    UpdateWindow(hDlg);
  }
}
