export type Turbin3Prereq = {
  version: '0.1.0';
  name: 'Turbin3_prereq';
  address: 'Trb3aEx85DW1cEEvoqEaBkMn1tfmNEEEPaKzLSu4YAv';
  metadata: {
    name: 'turbine_prereq';
    version: '0.1.0';
    spec: '0.1.0';
    description: 'Created with Anchor';
  };
  instructions: [
    {
      name: 'clean';
      discriminator: [250, 191, 56, 128, 150, 251, 1, 103];
      accounts: [
        {
          name: 'signer';
          writable: true;
          signer: true;
        },
        {
          name: 'prereq';
          writable: true;
        }
      ];
      args: [];
    },
    {
      name: 'submit';
      discriminator: [88, 166, 102, 181, 162, 127, 170, 48];
      accounts: [
        {
          name: 'signer';
          writable: true;
          signer: true;
        },
        {
          name: 'prereq';
          writable: true;
          pda: {
            seeds: [
              {
                kind: 'const';
                value: [112, 114, 101, 81, 50, 50, 53];
              },
              {
                kind: 'account';
                path: 'signer';
              }
            ];
          };
        },
        {
          name: 'system_program';
          address: '11111111111111111111111111111111';
        }
      ];
      args: [
        {
          name: 'github_username';
          type: 'bytes';
        }
      ];
    },
    {
      name: 'update';
      discriminator: [219, 200, 88, 176, 158, 63, 253, 127];
      accounts: [
        {
          name: 'signer';
          writable: true;
          signer: true;
        },
        {
          name: 'prereq';
          writable: true;
        },
        {
          name: 'system_program';
          address: '11111111111111111111111111111111';
        }
      ];
      args: [
        {
          name: 'github';
          type: 'bytes';
        }
      ];
    }
  ];
  accounts: [
    {
      name: 'Q2Prereq2024';
      discriminator: [210, 203, 168, 103, 251, 233, 204, 6];
    },
    {
      name: 'Q2Prereq2025';
      discriminator: [1, 231, 212, 91, 204, 178, 112, 25];
    }
  ];
  errors: [
    {
      code: 6000;
      name: 'InvalidGithubAccount';
      msg: 'Invalid Github account';
    }
  ];
  types: [
    {
      name: 'Q2Prereq2024';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'github';
            type: 'bytes';
          },
          {
            name: 'key';
            type: 'pubkey';
          }
        ];
      };
    },
    {
      name: 'Q2Prereq2025';
      type: {
        kind: 'struct';
        fields: [
          {
            name: 'github';
            type: 'bytes';
          },
          {
            name: 'key';
            type: 'pubkey';
          }
        ];
      };
    }
  ];
};
export const IDL: Turbin3Prereq = {
  version: '0.1.0',
  name: 'Turbin3_prereq',
  address: 'Trb3aEx85DW1cEEvoqEaBkMn1tfmNEEEPaKzLSu4YAv',
  metadata: {
    name: 'turbine_prereq',
    version: '0.1.0',
    spec: '0.1.0',
    description: 'Created with Anchor',
  },
  instructions: [
    {
      name: 'clean',
      discriminator: [250, 191, 56, 128, 150, 251, 1, 103],
      accounts: [
        {
          name: 'signer',
          writable: true,
          signer: true,
        },
        {
          name: 'prereq',
          writable: true,
        },
      ],
      args: [],
    },
    {
      name: 'submit',
      discriminator: [88, 166, 102, 181, 162, 127, 170, 48],
      accounts: [
        {
          name: 'signer',
          writable: true,
          signer: true,
        },
        {
          name: 'prereq',
          writable: true,
          pda: {
            seeds: [
              {
                kind: 'const',
                value: [112, 114, 101, 81, 50, 50, 53],
              },
              {
                kind: 'account',
                path: 'signer',
              },
            ],
          },
        },
        {
          name: 'system_program',
          address: '11111111111111111111111111111111',
        },
      ],
      args: [
        {
          name: 'github_username',
          type: 'bytes',
        },
      ],
    },
    {
      name: 'update',
      discriminator: [219, 200, 88, 176, 158, 63, 253, 127],
      accounts: [
        {
          name: 'signer',
          writable: true,
          signer: true,
        },
        {
          name: 'prereq',
          writable: true,
        },
        {
          name: 'system_program',
          address: '11111111111111111111111111111111',
        },
      ],
      args: [
        {
          name: 'github',
          type: 'bytes',
        },
      ],
    },
  ],
  accounts: [
    {
      name: 'Q2Prereq2024',
      discriminator: [210, 203, 168, 103, 251, 233, 204, 6],
    },
    {
      name: 'Q2Prereq2025',
      discriminator: [1, 231, 212, 91, 204, 178, 112, 25],
    },
  ],
  errors: [
    {
      code: 6000,
      name: 'InvalidGithubAccount',
      msg: 'Invalid Github account',
    },
  ],
  types: [
    {
      name: 'Q2Prereq2024',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'github',
            type: 'bytes',
          },
          {
            name: 'key',
            type: 'pubkey',
          },
        ],
      },
    },
    {
      name: 'Q2Prereq2025',
      type: {
        kind: 'struct',
        fields: [
          {
            name: 'github',
            type: 'bytes',
          },
          {
            name: 'key',
            type: 'pubkey',
          },
        ],
      },
    },
  ],
};
