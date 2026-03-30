/*
 * func-name: sub_1400024D4
 * func-address: 0x1400024d4
 * callers: 0x140003cec
 * callees: none
 */

BOOL __fastcall sub_1400024D4(LPARAM lParam)
{
  SendMessageW(qword_140010748, 0x40Bu, 1u, lParam);
  return UpdateWindow(qword_140010748);
}
