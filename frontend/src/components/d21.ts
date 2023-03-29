export type D21 = {
  "version": "0.1.0",
  "name": "d21",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "basicInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "addSubject",
      "accounts": [
        {
          "name": "subject",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "basicInfo",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        }
      ]
    },
    {
      "name": "addVoter",
      "accounts": [
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "basicInfo",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "voter",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "vote",
      "accounts": [
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "subject",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "basicInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "subject",
          "type": "publicKey"
        },
        {
          "name": "isPositiveVote",
          "type": "bool"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "subjectAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "votes",
            "type": "i64"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "pubkey",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "voterAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "votedNegativelyOnce",
            "type": "bool"
          },
          {
            "name": "secondVoteAddress",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "firstVoteAddress",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "basicInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "endDate",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NameTooLong",
      "msg": "Specified name for subject is too long."
    },
    {
      "code": 6001,
      "name": "ElectionEnded",
      "msg": "Election has already ended."
    },
    {
      "code": 6002,
      "name": "NotOwner",
      "msg": "You are not the owner of this election."
    },
    {
      "code": 6003,
      "name": "VoteForSameSubjectTwice",
      "msg": "You have already voted for this subject."
    },
    {
      "code": 6004,
      "name": "NoMorePositiveVotes",
      "msg": "You have no more positive votes left."
    },
    {
      "code": 6005,
      "name": "NoMoreNegativeVotes",
      "msg": "You have already voted negatively once."
    },
    {
      "code": 6006,
      "name": "NegativeVotesAfterTwoPositive",
      "msg": "You can only vote negatively after voting positively twice."
    },
    {
      "code": 6007,
      "name": "InvalidBump",
      "msg": "Could not find bump value."
    }
  ]
};

export const IDL: D21 = {
  "version": "0.1.0",
  "name": "d21",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [
        {
          "name": "basicInfo",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "addSubject",
      "accounts": [
        {
          "name": "subject",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "basicInfo",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        }
      ]
    },
    {
      "name": "addVoter",
      "accounts": [
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "basicInfo",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "voter",
          "type": "publicKey"
        }
      ]
    },
    {
      "name": "vote",
      "accounts": [
        {
          "name": "voter",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "subject",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "basicInfo",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "initializer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "subject",
          "type": "publicKey"
        },
        {
          "name": "isPositiveVote",
          "type": "bool"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "subjectAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "votes",
            "type": "i64"
          },
          {
            "name": "name",
            "type": "string"
          },
          {
            "name": "pubkey",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "voterAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "votedNegativelyOnce",
            "type": "bool"
          },
          {
            "name": "secondVoteAddress",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "firstVoteAddress",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "basicInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "endDate",
            "type": "i64"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NameTooLong",
      "msg": "Specified name for subject is too long."
    },
    {
      "code": 6001,
      "name": "ElectionEnded",
      "msg": "Election has already ended."
    },
    {
      "code": 6002,
      "name": "NotOwner",
      "msg": "You are not the owner of this election."
    },
    {
      "code": 6003,
      "name": "VoteForSameSubjectTwice",
      "msg": "You have already voted for this subject."
    },
    {
      "code": 6004,
      "name": "NoMorePositiveVotes",
      "msg": "You have no more positive votes left."
    },
    {
      "code": 6005,
      "name": "NoMoreNegativeVotes",
      "msg": "You have already voted negatively once."
    },
    {
      "code": 6006,
      "name": "NegativeVotesAfterTwoPositive",
      "msg": "You can only vote negatively after voting positively twice."
    },
    {
      "code": 6007,
      "name": "InvalidBump",
      "msg": "Could not find bump value."
    }
  ]
};
