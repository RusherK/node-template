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

  const fetchKittyCnt = () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
    api.query.kittiesModule.kittiesCount(amount => {
      //获取猫咪总数/ID
      let kittyCount = amount.toJSON();
      setKittyCnt(kittyCount);

      fetchKittiesDna(kittyCount);
      fetchKittiesOwner(kittyCount)

    })
  };

  const fetchKittiesDna = (kCnt) => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
    //获取猫咪们的dna
    //Q: 猫咪会死吗？ KittyIndex是否按顺序递增
    api.query.kittiesModule.kitties.multi([...Array(kCnt).keys()], (data) => {
      let tempData = [];
      data.map(row => {
        if (row.isNone) {
          tempData.push('猫不存在');
        } else {
          //Option 转字符串
          //hash.toJSON() 或 hash.toHuman() 
          //value.toJSON() 或 value.toHuman()
          let kittyDna = row.value.toHuman();

          tempData.push(kittyDna);
        }
      })
      setKittyDNAs(tempData);
    })
  }

  const fetchKittiesOwner = (kCnt) => {
    //获取猫咪的主人

    api.query.kittiesModule.kittyOwners.multi([...Array(kCnt).keys()], (data) => {
      let tempData = [];
      data.map(row => {
        if (row.isNone) {
          tempData.push('猫不存在');
        } else {
          //Option 转字符串
          //hash.toJSON() 或 hash.toHuman() 
          //value.toJSON() 或 value.toHuman()
          let kittyOwner = row.value.toHuman();

          tempData.push(kittyOwner);
        }
      })
      setKittyOwners(tempData);
    })
  }

  const fetchKitties = () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
    //combine

    let kittiesAllInfo = [];
    for (let idx = 0; idx < kittyCnt; idx++) {
      kittiesAllInfo.push({
        id: idx,
        dna: kittyDNAs[idx],
        owner: kittyOwners[idx]
      })
    }

    setKitties(kittiesAllInfo);
  }

  const populateKitties = () => {
    /* TODO: 加代码，从 substrate 端读取数据过来 */
  };

  useEffect(fetchKittyCnt, [api, keyring]);
  useEffect(fetchKitties, [api, kittyDNAs, kittyOwners]);
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
