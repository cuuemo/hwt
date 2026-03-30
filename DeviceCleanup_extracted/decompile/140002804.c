/*
 * func-name: sub_140002804
 * func-address: 0x140002804
 * callers: 0x140004534, 0x1400047fc
 * callees: 0x140007530
 */

BOOL sub_140002804()
{
  int v0; // ebx
  HDWP v1; // rax
  HDWP v2; // rax
  HDWP v3; // rax
  struct tagRECT cy; // [rsp+40h] [rbp-48h] BYREF
  struct tagRECT v6; // [rsp+50h] [rbp-38h] BYREF
  struct tagRECT v7; // [rsp+60h] [rbp-28h] BYREF
  struct tagRECT Rect; // [rsp+70h] [rbp-18h] BYREF

  GetWindowRect(hDlg, &Rect);
  GetClientRect(hDlg, &cy);
  GetWindowRect(qword_140010748, &v6);
  sub_140007530(hDlg, &v6);
  GetWindowRect(hWnd, &v7);
  sub_140007530(hDlg, &v7);
  v0 = v6.bottom - v6.top;
  v1 = BeginDeferWindowPos(2);
  v2 = DeferWindowPos(v1, qword_140010748, 0, 0, cy.bottom - v0, cy.right, cy.bottom, 4u);
  v3 = DeferWindowPos(v2, hWnd, 0, 0, y, cy.right + 2, cy.bottom - y - v0, 4u);
  EndDeferWindowPos(v3);
  return UpdateWindow(hWnd);
}
