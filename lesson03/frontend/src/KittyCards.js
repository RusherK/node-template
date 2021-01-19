import React from 'react';
import { Card, Grid, Message, Modal, Form, Label } from 'semantic-ui-react';

import KittyAvatar from './KittyAvatar';
import { TxButton } from './substrate-lib/components';

import { Button, Row, Col, Divider, Tag } from "antd";
import "antd/dist/antd.css";
// --- About Modal ---

const TransferModal = props => {
  const { kitty, accountPair, setStatus } = props;
  const [open, setOpen] = React.useState(false);
  const [formValue, setFormValue] = React.useState({});

  const formChange = key => (ev, el) => {
    /* TODO: 加代码 */

    setFormValue({ [key]: el.value });
  };

  const confirmAndClose = (unsub) => {
    unsub();
    setOpen(false);
  };

  return <Modal onClose={() => setOpen(false)} onOpen={() => setOpen(true)} open={open}
    trigger={<Button basic color='blue'>转让</Button>}>
    <Modal.Header>毛孩转让</Modal.Header>
    <Modal.Content><Form>
      <Form.Input fluid label='毛孩 ID' readOnly value={kitty.id} />
      <Form.Input fluid label='转让对象' placeholder='对方地址' onChange={formChange('target')} />
    </Form></Modal.Content>
    <Modal.Actions>
      <Button basic color='grey' onClick={() => setOpen(false)}>取消</Button>
      <TxButton
        accountPair={accountPair} label='确认转让' type='SIGNED-TX' setStatus={setStatus}
        onClick={confirmAndClose}
        attrs={{
          palletRpc: 'kittiesModule',
          callable: 'transfer',
          inputParams: [formValue.target, kitty.id],
          paramFields: [true, true]
        }}
      />
    </Modal.Actions>
  </Modal>;
};

// --- About Kitty Card ---

const KittyCard = props => {
  /*
    TODO: 加代码。这里会 UI 显示一张 `KittyCard` 是怎么样的。这里会用到：
    ```
    <KittyAvatar dna={dna} /> - 来描绘一只猫咪
    <TransferModal kitty={kitty} accountPair={accountPair} setStatus={setStatus}/> - 来作转让的弹出层
    ```
  */


  const { dna } = props
  //把hash字符串转为纯数字，用来获取猫咪特征
  let pureNum = dna.replace(/[^0-9]/ig, "");

  return (
    <div><KittyAvatar dna={pureNum} />
    </div>
  );
};

const KittyCards = props => {
  const { kitties, accountPair, setStatus } = props;
  console.log()
  const kittiesStyle = { padding: "20px", marginBottom: "20px", borderRadius: "8px", boxShadow: "0px 0px 10px #ddd" };
  const textCenter = { textAlign: "center" };
  const inlineText = { wordBreak: "break-all", color: "#999" }
  const tagStyle = { position: "absolute", right: "-30px", top: "-10px", borderRadius: "3px" }
  /* TODO: 加代码。这里会枚举所有的 `KittyCard` */
  return (
    <Row justify="space-between">
      {
        kitties.map((item, idx) => {
          return (
            <Col className="gutter-row" style={kittiesStyle} span={7} key={idx}>
              {
                accountPair && accountPair.address == item.owner ?
                  <Tag style={tagStyle} color="#2db7f5">我的</Tag>
                  : null
              }
              <KittyCard dna={item.dna} ></KittyCard>
              <Divider />

              <div>ID号：{idx}</div>
              <div>基因：</div>
              <div style={inlineText}>{item.dna}</div>
              <div>主人：</div>
              <div style={inlineText}>{item.owner}</div>

              <Divider />
              <div style={textCenter}>
                <TransferModal kitty={item} accountPair={accountPair} setStatus={setStatus} />
              </div>
            </Col>
          )
        })
      }
    </Row>
  );
};

export default KittyCards;
