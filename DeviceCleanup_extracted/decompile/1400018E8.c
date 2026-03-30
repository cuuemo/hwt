/*
 * func-name: sub_1400018E8
 * func-address: 0x1400018e8
 * callers: 0x140001dc0, 0x140006850, 0x1400070b8
 * callees: none
 */

const char *__fastcall sub_1400018E8(unsigned int a1)
{
  const CHAR *v2; // rdx

  if ( a1 > 0x32 )
  {
    if ( a1 > 0x3EF )
    {
      if ( a1 > 0x45D )
      {
        if ( a1 > 0x522 )
        {
          switch ( a1 )
          {
            case 0x54Bu:
              return "NO_SUCH_DOMAIN";
            case 0x54Fu:
              return "INTERNAL_ERROR";
            case 0x5B4u:
              return "TIMEOUT";
            case 0xE000020B:
              return "NO_SUCH_DEVINST";
          }
        }
        else
        {
          switch ( a1 )
          {
            case 0x522u:
              return "PRIVILEGE_NOT_HELD";
            case 0x48Fu:
              return "DEVICE_NOT_CONNECTED";
            case 0x4BCu:
              return "INVALID_DOMAINNAME";
            case 0x4DDu:
              return "NOT_LOGGED_ON";
            case 0x520u:
              return "NO_SUCH_LOGON_SESSION";
            case 0x521u:
              return "NO_SUCH_PRIVILEGE";
          }
        }
      }
      else
      {
        if ( a1 == 1117 )
          return "IO_DEVICE";
        if ( a1 > 0x421 )
        {
          switch ( a1 )
          {
            case 0x422u:
              return "SERVICE_DISABLED";
            case 0x424u:
              return "SERVICE_DOES_NOT_EXIST";
            case 0x425u:
              return "SERVICE_CANNOT_ACCEPT_CTRL";
            case 0x426u:
              return "SERVICE_NOT_ACTIVE";
          }
        }
        else
        {
          switch ( a1 )
          {
            case 0x421u:
              return "INVALID_SERVICE_ACCOUNT";
            case 0x3F0u:
              return "NO_TOKEN";
            case 0x41Cu:
              return "INVALID_SERVICE_CONTROL";
            case 0x41Du:
              return "SERVICE_REQUEST_TIMEOUT";
            case 0x41Fu:
              return "SERVICE_DATABASE_LOCKED";
            case 0x420u:
              return "SERVICE_ALREADY_RUNNING";
          }
        }
      }
    }
    else
    {
      if ( a1 == 1007 )
        return "FULLSCREEN_MODE";
      if ( a1 > 0xE8 )
      {
        if ( a1 > 0x3E3 )
        {
          switch ( a1 )
          {
            case 0x3E5u:
              return "IO_PENDING";
            case 0x3ECu:
              return "INVALID_FLAGS";
            case 0x3EDu:
              return "UNRECOGNIZED_VOLUME";
            case 0x3EEu:
              return "FILE_INVALID";
          }
        }
        else
        {
          switch ( a1 )
          {
            case 0x3E3u:
              return "OPERATION_ABORTED";
            case 0xE9u:
              return "PIPE_NOT_CONNECTED";
            case 0xEAu:
              return "MORE_DATA";
            case 0x1B1u:
              return "NO_SUCH_DEVICE";
            case 0x217u:
              return "PIPE_CONNECTED";
            case 0x218u:
              return "PIPE_LISTENING";
          }
        }
      }
      else
      {
        if ( a1 == 232 )
          return "NO_DATA";
        if ( a1 > 0xA1 )
        {
          switch ( a1 )
          {
            case 0xAAu:
              return "BUSY";
            case 0xB7u:
              return "ALREADY_EXISTS";
            case 0xE6u:
              return "BAD_PIPE";
            case 0xE7u:
              return "PIPE_BUSY";
          }
        }
        else
        {
          switch ( a1 )
          {
            case 0xA1u:
              return "BAD_PATHNAME";
            case 0x35u:
              return "BAD_NETPATH";
            case 0x57u:
              return "INVALID_PARAMETER";
            case 0x70u:
              return "DISK_FULL";
            case 0x79u:
              return "SEM_TIMEOUT";
            case 0x7Bu:
              return "INVALID_NAME";
          }
        }
      }
    }
  }
  else
  {
    if ( a1 == 50 )
      return "NOT_SUPPORTED";
    if ( a1 > 0xA )
    {
      switch ( a1 )
      {
        case 0xDu:
          return "INVALID_DATA";
        case 0x12u:
          return "NO_MORE_FILES";
        case 0x13u:
          return "WRITE_PROTECT";
        case 0x15u:
          return "NOT_READY";
        case 0x1Fu:
          return "GEN_FAILURE";
        case 0x20u:
          return "SHARING_VIOLATION";
      }
    }
    else
    {
      switch ( a1 )
      {
        case 0xAu:
          return "BAD_ENVIRONMENT";
        case 0u:
          return "ok";
        case 1u:
          return "INVALID_FUNCTION";
        case 2u:
          return "FILE_NOT_FOUND";
        case 3u:
          return "PATH_NOT_FOUND";
        case 5u:
          return "ACCESS_DENIED";
        case 6u:
          return "INVALID_HANDLE";
        case 8u:
          return "NOT_ENOUGH_MEMORY";
      }
    }
  }
  v2 = "0x%X";
  if ( a1 < 0x10000000 )
    v2 = "%u";
  wsprintfA(byte_14000FB88, v2, a1);
  return byte_14000FB88;
}
