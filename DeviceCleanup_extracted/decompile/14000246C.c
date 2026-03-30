/*
 * func-name: sub_14000246C
 * func-address: 0x14000246c
 * callers: 0x140002508, 0x1400036d4, 0x140003cec
 * callees: none
 */

LRESULT sub_14000246C()
{
  LRESULT result; // rax

  result = SendMessageW(hWnd, 0x1032u, 0, 0);
  if ( (_DWORD)result != dword_14000F678 )
  {
    dword_14000F678 = result;
    wsprintfW(&lParam, L" Selected Devices: %i", (unsigned int)result);
    return SendMessageW(qword_140010748, 0x40Bu, 1u, (LPARAM)&lParam);
  }
  return result;
}
