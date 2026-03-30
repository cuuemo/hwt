/*
 * func-name: sub_140002008
 * func-address: 0x140002008
 * callers: 0x140003c1c, 0x140003cec
 * callees: 0x140001854, 0x140007a10, 0x140007db0, 0x1400085c4
 */

int sub_140002008()
{
  int result; // eax
  int v1; // edi
  char *v2; // rbx
  CHAR String[32]; // [rsp+30h] [rbp-78h] BYREF
  char v4; // [rsp+50h] [rbp-58h] BYREF
  WINDOWPLACEMENT wndpl; // [rsp+70h] [rbp-38h] BYREF

  result = IsWindowVisible(hDlg);
  if ( result )
  {
    wndpl.length = 44;
    sub_140001854(&wndpl.flags, 0, 0x28u);
    if ( GetWindowPlacement(hDlg, &wndpl) )
      WritePrivateProfileStructA("Settings", "WindowPlacement", &wndpl, 0x2Cu, MultiByteStr);
    sub_1400085C4();
    sub_140007DB0("Settings", "DPI");
    sub_140007DB0("Settings", "VisualStyles");
    wsprintfA(String, "%i", dword_140010790);
    WritePrivateProfileStringA("Settings", "SortCol", String, MultiByteStr);
    wsprintfA(String, "%i", dword_14000F478[dword_140010790]);
    WritePrivateProfileStringA("Settings", "SortDir", String, MultiByteStr);
    sub_140007DB0("Settings", "ShowEnumerator");
    sub_140007DB0("Settings", "ShowService");
    sub_140007DB0("Settings", "ShowComPort");
    sub_140007DB0("Settings", "ShowDeviceId");
    sub_140007DB0("Settings", "ListDevsRoot");
    sub_140007DB0("Settings", "ListDevsSwd");
    sub_140007DB0("Settings", "ListDevsSw");
    v1 = 0;
    if ( (int)sub_140007A10(hWnd) > 0 )
    {
      v2 = &v4;
      do
      {
        SendMessageW(hWnd, 0x101Du, v1++, 0);
        v2 += 4;
      }
      while ( v1 < (int)sub_140007A10(hWnd) );
    }
    nNumber = SendMessageW(hWnd, 0x101Du, 0, 0);
    dword_1400107A8 = SendMessageW(hWnd, 0x101Du, 1u, 0);
    dword_1400107AC = SendMessageW(hWnd, 0x101Du, 2u, 0);
    dword_1400107B0 = SendMessageW(hWnd, 0x101Du, dword_14000F464, 0);
    dword_1400107B4 = SendMessageW(hWnd, 0x101Du, dword_14000F468, 0);
    dword_1400107B8 = SendMessageW(hWnd, 0x101Du, dword_14000F46C, 0);
    dword_1400107BC = SendMessageW(hWnd, 0x101Du, dword_14000F470, 0);
    sub_140007DB0("Settings", "DeviceNameColumnWidth");
    sub_140007DB0("Settings", "LastUsedColumnWidth");
    result = sub_140007DB0("Settings", "ClassColumnWidth");
    if ( dword_1400107B0 )
      result = sub_140007DB0("Settings", "EnumeratorColumnWidth");
    if ( dword_1400107B4 )
      result = sub_140007DB0("Settings", "ServiceColumnWidth");
    if ( dword_1400107B8 )
      result = sub_140007DB0("Settings", "ComPortColumnWidth");
    if ( dword_1400107BC )
      return sub_140007DB0("Settings", "DeviceIdColumnWidth");
  }
  return result;
}
