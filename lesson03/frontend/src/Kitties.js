import React, { useEffect, useState } from 'react';
import { Form, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

import KittyCards from './KittyCards';

export default function Kitties(props) {
  const { api, keyring } = useSubstrate();
  const { accountPair } = props;

  const [kittyCnt, setKittyCnt] = useState(0);
  const [kittyDNAs, setKittyDNAs] = useState([]);
  const [kittyOwners, setKittyOwners] = useState([]);
  const [kittyPrices, setKittyPrices] = useState([]);
  const [kitties, setKitties] = useState([]);
  const [status, setStatus] = useState('');

  const fetchKittyCnt = async () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
    const unsub = await api.query.kittiesModule.kittiesCount(amount => {
      //获取猫咪总数/ID
      let kittyCount = amount.toJSON();
      setKittyCnt(kittyCount);

    })
  };


  const fetchKitties = async () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */

    const unsub = await api.queryMuilt([
      [api.query.kittiesModule.kitties, kittyCnt],
      [api.query.kittiesModule.kittyOwners, kittyCnt],
    ], (dnaOption, ownerOption) => {
      //Option 转字符串
      //hash.toJSON() 或 hash.toHuman() 
      //value.toJSON() 或 value.toHuman()
      //dna arr
      let dnaData = [];
      dnaOption.map(item => {
        let kittyDna = item.value.toHuman();
        dnaData.push(kittyDna);
      })
      // setKittyDNAs(dnaData);

      //owner arr
      let ownerData = [];
      ownerOption.map(item => {
        let ownerDna = item.value.toHuman();
        ownerData.push(ownerDna);
      })
      // setKittyOwners(ownerData);

      let kittiesAllInfo = [];

      for (let idx = 0; idx < kittyCnt; idx++) {
        kittiesAllInfo.push({
          id: idx,
          dna: dnaData[idx],
          owner: ownerData[idx]
        })
      }

      setKitties(kittiesAllInfo);

    })
  }

  const populateKitties = () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
  };

  useEffect(fetchKittyCnt, [api, keyring]);
  useEffect(fetchKitties, [api, kittyCnt]);
  useEffect(populateKitties, [kittyDNAs, kittyOwners]);

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus} />
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>;
}
