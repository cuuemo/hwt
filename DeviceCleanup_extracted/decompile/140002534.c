/*
 * func-name: sub_140002534
 * func-address: 0x140002534
 * callers: 0x140003cec, 0x1400047fc
 * callees: 0x140001854
 */

LRESULT sub_140002534()
{
  int v0; // ebx
  LRESULT result; // rax
  WPARAM v2; // r8
  WPARAM v3; // r8
  int lParam; // [rsp+20h] [rbp-38h] BYREF
  LPARAM lParam_4; // [rsp+24h] [rbp-34h] BYREF
  int v6; // [rsp+28h] [rbp-30h]
  const wchar_t *v7; // [rsp+30h] [rbp-28h]

  SendMessageW(hWnd, 0x101Cu, 6u, 0);
  SendMessageW(hWnd, 0x101Cu, 5u, 0);
  SendMessageW(hWnd, 0x101Cu, 4u, 0);
  SendMessageW(hWnd, 0x101Cu, 3u, 0);
  v0 = 3;
  result = (LRESULT)sub_140001854(&lParam_4, 0, 0x24u);
  lParam = 14;
  if ( dword_14001079C )
  {
    v6 = dword_1400107B4;
    dword_14000F468 = 3;
    v0 = 4;
    v7 = L"Service";
    result = SendMessageW(hWnd, 0x1061u, 3u, (LPARAM)&lParam);
  }
  if ( dword_140010798 )
  {
    dword_14000F464 = v0;
    v2 = v0;
    v6 = dword_1400107B0;
    v7 = L"Enumerator";
    ++v0;
    result = SendMessageW(hWnd, 0x1061u, v2, (LPARAM)&lParam);
  }
  if ( dword_1400107A0 )
  {
    dword_14000F46C = v0;
    v3 = v0;
    v6 = dword_1400107B8;
    v7 = L"COM Port";
    ++v0;
    result = SendMessageW(hWnd, 0x1061u, v3, (LPARAM)&lParam);
  }
  if ( dword_140010794 )
  {
    v6 = dword_1400107BC;
    dword_14000F470 = v0;
    v7 = L"Device ID";
    return SendMessageW(hWnd, 0x1061u, v0, (LPARAM)&lParam);
  }
  return result;
}
