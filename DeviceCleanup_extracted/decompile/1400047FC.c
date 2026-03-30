/*
 * func-name: sub_1400047FC
 * func-address: 0x1400047fc
 * callers: 0x1400052dc
 * callees: 0x140001854, 0x140001e04, 0x140002534, 0x140002804, 0x140003bcc, 0x140007478, 0x140007530, 0x1400075d4, 0x140007a40, 0x140007a78, 0x140007d54, 0x1400085c4, 0x1400086f4
 */

__int64 __fastcall sub_1400047FC(UINT a1)
{
  int WindowTextW; // eax
  HMENU v3; // rcx
  UINT v4; // edx
  int v5; // eax
  HWND v6; // rbx
  HWND DlgItem; // rax
  HWND v8; // rax
  unsigned int v9; // eax
  int v10; // edi
  UINT PrivateProfileIntA; // ebx
  INT *p_Struct; // rsi
  __int64 v13; // r12
  UINT v14; // eax
  __int64 v15; // rdx
  HWND v16; // rcx
  UINT showCmd; // eax
  HWND DesktopWindow; // rax
  struct tagRECT v20; // [rsp+30h] [rbp-178h] BYREF
  INT Struct; // [rsp+40h] [rbp-168h] BYREF
  INT nDefault; // [rsp+44h] [rbp-164h]
  INT v23; // [rsp+48h] [rbp-160h]
  INT v24; // [rsp+4Ch] [rbp-15Ch]
  INT v25; // [rsp+50h] [rbp-158h]
  INT v26; // [rsp+54h] [rbp-154h]
  INT v27; // [rsp+58h] [rbp-150h]
  int v28; // [rsp+5Ch] [rbp-14Ch]
  WINDOWPLACEMENT wndpl; // [rsp+60h] [rbp-148h] BYREF
  struct tagRECT v30; // [rsp+90h] [rbp-118h] BYREF
  int v31; // [rsp+A0h] [rbp-108h] BYREF
  LPARAM v32; // [rsp+A4h] [rbp-104h] BYREF
  int v33; // [rsp+A8h] [rbp-100h]
  const wchar_t *v34; // [rsp+B0h] [rbp-F8h]
  LPARAM lParam; // [rsp+C8h] [rbp-E0h] BYREF
  __int64 v36; // [rsp+D0h] [rbp-D8h]
  int v37; // [rsp+D8h] [rbp-D0h]
  struct tagRECT v38; // [rsp+E0h] [rbp-C8h] BYREF
  struct tagRECT Rect; // [rsp+F0h] [rbp-B8h] BYREF
  WCHAR String; // [rsp+100h] [rbp-A8h] BYREF
  _BYTE v41[126]; // [rsp+102h] [rbp-A6h] BYREF

  wndpl.length = 44;
  sub_140001854(&wndpl.flags, 0, 0x28u);
  InitCommonControls();
  hDlg = CreateDialogParamW(hInstance, (LPCWSTR)0x65, 0, (DLGPROC)sub_140004704, 0);
  hWnd = GetDlgItem(hDlg, 1000);
  qword_140010748 = GetDlgItem(hDlg, 1001);
  if ( GetPrivateProfileIntA("Settings", "ShowVerInCaption", 1, MultiByteStr) )
  {
    String = 0;
    sub_140001854(v41, 0, 0x7Eu);
    WindowTextW = GetWindowTextW(hDlg, &String, 63);
    if ( WindowTextW )
    {
      wsprintfW((LPWSTR)&v41[2 * WindowTextW - 2], L" V%s", &unk_1400106EC);
      SetWindowTextW(hDlg, &String);
    }
  }
  if ( !dword_1400107CC )
  {
    GetWindowTextW(hDlg, &String, 64);
    lstrcatW(&String, L"  [Restricted]");
    SetWindowTextW(hDlg, &String);
  }
  hmenu = GetMenu(hDlg);
  qword_140010638 = CreateMenu();
  hMenu = CreateMenu();
  AppendMenuW(hMenu, 0, 0xA029u, L"&Remove Device");
  AppendMenuW(hMenu, 0x800u, 0, L"-");
  AppendMenuW(hMenu, 0, 0xA02Au, L"&Copy");
  AppendMenuW(hMenu, 0x800u, 0, L"-");
  AppendMenuW(hMenu, 0, 0xA02Bu, L"&Properties");
  AppendMenuW(qword_140010638, 0x10u, (UINT_PTR)hMenu, L"&Pop");
  SetMenuDefaultItem(hMenu, 0xA02Bu, 0);
  if ( !(unsigned int)sub_1400075D4() )
    sub_140007D54(hmenu, 0x9CA6u);
  v3 = hmenu;
  if ( dword_1400107CC )
  {
    v4 = 40101;
  }
  else
  {
    sub_140007D54(hmenu, 0x9CA6u);
    sub_140007D54(hmenu, 0x9CAFu);
    v3 = hMenu;
    v4 = 41001;
  }
  sub_140007D54(v3, v4);
  v5 = sub_1400086F4(180);
  HIDWORD(lParam) = -1;
  LODWORD(lParam) = v5;
  v36 = 0;
  v37 = 0;
  SendMessageW(qword_140010748, 0x404u, 2u, (LPARAM)&lParam);
  v6 = hDlg;
  GetClientRect(hDlg, &Rect);
  DlgItem = GetDlgItem(v6, 1000);
  GetWindowRect(DlgItem, &v38);
  sub_140007530(v6, &v38);
  y = v38.top;
  v8 = GetDlgItem(v6, 1000);
  dword_14000FFF8 = sub_140007A40(v8);
  dword_140010798 = GetPrivateProfileIntA("Settings", "ShowEnumerator", 1, MultiByteStr);
  dword_14001079C = GetPrivateProfileIntA("Settings", "ShowService", 1, MultiByteStr);
  dword_1400107A0 = GetPrivateProfileIntA("Settings", "ShowComPort", 1, MultiByteStr);
  dword_140010794 = GetPrivateProfileIntA("Settings", "ShowDeviceId", 0, MultiByteStr);
  sub_140001E04(0x9D09u, dword_140010798);
  sub_140001E04(0x9D08u, dword_14001079C);
  sub_140001E04(0x9D0Au, dword_1400107A0);
  sub_140001E04(0x9D0Bu, dword_140010794);
  dword_1400107C0 = GetPrivateProfileIntA("Settings", "ListDevsRoot", 0, MultiByteStr);
  dword_1400107C4 = GetPrivateProfileIntA("Settings", "ListDevsSwd", 1, MultiByteStr);
  dword_1400107C8 = GetPrivateProfileIntA("Settings", "ListDevsSw", 1, MultiByteStr);
  sub_140001E04(0x9D6Du, dword_1400107C0);
  sub_140001E04(0x9D6Eu, dword_1400107C4);
  sub_140001E04(0x9D6Fu, dword_1400107C8);
  v9 = SendMessageW(hWnd, 0x1037u, 0, 0);
  SendMessageW(hWnd, 0x1036u, 0, v9 | 0x20LL);
  v10 = sub_1400085C4();
  PrivateProfileIntA = GetPrivateProfileIntA("Settings", "DPI", 0, MultiByteStr);
  dword_14000F450 = sub_1400086F4(16);
  Struct = 330;
  cy = sub_1400086F4(32);
  v26 = 80;
  v27 = 400;
  p_Struct = &Struct;
  v13 = 8;
  nDefault = 100;
  v23 = 100;
  v24 = 100;
  v25 = 100;
  v28 = 100;
  do
  {
    *p_Struct = sub_1400086F4((unsigned int)*p_Struct);
    ++p_Struct;
    --v13;
  }
  while ( v13 );
  GetPrivateProfileStructA("Settings", "ColumnWidths", &Struct, 0x20u, MultiByteStr);
  WritePrivateProfileStringA("Settings", "ColumnWidths", 0, MultiByteStr);
  nNumber = GetPrivateProfileIntA("Settings", "DeviceNameColumnWidth", Struct, MultiByteStr);
  dword_1400107A8 = GetPrivateProfileIntA("Settings", "LastUsedColumnWidth", nDefault, MultiByteStr);
  dword_1400107AC = GetPrivateProfileIntA("Settings", "ClassColumnWidth", v23, MultiByteStr);
  dword_1400107B0 = GetPrivateProfileIntA("Settings", "EnumeratorColumnWidth", v24, MultiByteStr);
  dword_1400107B4 = GetPrivateProfileIntA("Settings", "ServiceColumnWidth", v25, MultiByteStr);
  dword_1400107B8 = GetPrivateProfileIntA("Settings", "ComPortColumnWidth", v26, MultiByteStr);
  dword_1400107BC = GetPrivateProfileIntA("Settings", "DeviceIdColumnWidth", v27, MultiByteStr);
  if ( PrivateProfileIntA && v10 != PrivateProfileIntA )
  {
    nNumber = MulDiv(nNumber, v10, PrivateProfileIntA);
    dword_1400107A8 = MulDiv(dword_1400107A8, v10, PrivateProfileIntA);
    dword_1400107AC = MulDiv(dword_1400107AC, v10, PrivateProfileIntA);
    dword_1400107B0 = MulDiv(dword_1400107B0, v10, PrivateProfileIntA);
    dword_1400107B4 = MulDiv(dword_1400107B4, v10, PrivateProfileIntA);
    dword_1400107B8 = MulDiv(dword_1400107B8, v10, PrivateProfileIntA);
    dword_1400107BC = MulDiv(dword_1400107BC, v10, PrivateProfileIntA);
  }
  sub_140001854(&v32, 0, 0x24u);
  v33 = nNumber;
  v34 = L"Device Name";
  v31 = 14;
  SendMessageW(hWnd, 0x1061u, 0, (LPARAM)&v31);
  v33 = dword_1400107A8;
  v34 = L"Last used";
  SendMessageW(hWnd, 0x1061u, 1u, (LPARAM)&v31);
  v33 = dword_1400107AC;
  v34 = L"Class";
  SendMessageW(hWnd, 0x1061u, 2u, (LPARAM)&v31);
  sub_140002534();
  ClassImageListData.cbSize = 24;
  SetupDiGetClassImageList(&ClassImageListData);
  SendMessageW(hWnd, 0x1003u, 1u, (LPARAM)ClassImageListData.ImageList);
  dword_140010790 = GetPrivateProfileIntA("Settings", "SortCol", 0, MultiByteStr);
  v14 = GetPrivateProfileIntA("Settings", "SortDir", 1, MultiByteStr);
  v15 = dword_140010790;
  v16 = hWnd;
  dword_14000F478[dword_140010790] = v14;
  sub_140007A78(v16, v15, v14);
  if ( (unsigned int)sub_140007478() )
  {
    dword_14000F458 = GetPrivateProfileIntA("Settings", "VisualStyles", 1, MultiByteStr);
    sub_140001E04(0x9DD0u, dword_14000F458);
    if ( !dword_14000F458 )
      sub_140003BCC();
  }
  else
  {
    EnableMenuItem(hmenu, 0x9DD0u, 1u);
  }
  if ( GetPrivateProfileStructA("Settings", "WindowPlacement", &wndpl, 0x2Cu, MultiByteStr) )
  {
    showCmd = wndpl.showCmd;
    if ( wndpl.showCmd == 2 )
    {
      showCmd = wndpl.flags & 2 | 1;
      wndpl.showCmd = showCmd;
    }
    if ( a1 != 1 )
    {
      if ( a1 != 10 )
        showCmd = a1;
      wndpl.showCmd = showCmd;
    }
    if ( PrivateProfileIntA && PrivateProfileIntA < 0x3E8 && v10 != PrivateProfileIntA )
    {
      wndpl.rcNormalPosition.left = MulDiv(wndpl.rcNormalPosition.left, v10, PrivateProfileIntA);
      wndpl.rcNormalPosition.top = MulDiv(wndpl.rcNormalPosition.top, v10, PrivateProfileIntA);
      wndpl.rcNormalPosition.bottom = MulDiv(wndpl.rcNormalPosition.bottom, v10, PrivateProfileIntA);
      wndpl.rcNormalPosition.right = MulDiv(wndpl.rcNormalPosition.right, v10, PrivateProfileIntA);
    }
  }
  else
  {
    wndpl.showCmd = 1;
    DesktopWindow = GetDesktopWindow();
    GetWindowRect(DesktopWindow, &v30);
    GetWindowRect(hDlg, &v20);
    wndpl.rcNormalPosition.left = (v30.right + v20.left - v20.right - v30.left) / 2;
    wndpl.rcNormalPosition.right = v20.right + wndpl.rcNormalPosition.left - v20.left;
    wndpl.rcNormalPosition.top = (v30.bottom + v20.top - v20.bottom - v30.top) / 2;
    wndpl.rcNormalPosition.bottom = v20.bottom + wndpl.rcNormalPosition.top - v20.top;
  }
  SetWindowPlacement(hDlg, &wndpl);
  sub_140002804();
  ShowWindow(hDlg, wndpl.showCmd);
  SetFocus(hWnd);
  UpdateWindow(hDlg);
  SendMessageW(hDlg, 0xBu, 1u, 0);
  UpdateWindow(hDlg);
  SetTimer(hDlg, 0x12340001u, 0xAu, (TIMERPROC)sub_140003C04);
  return 1;
}
