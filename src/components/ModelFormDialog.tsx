import { forwardRef, useCallback, useImperativeHandle, useState } from 'react';

import { PROVIDER_OLLAMA } from '@/lib/constants';
import type { DialogHandler, Model, NewModel } from '@/lib/types';

import ModelFormDialogContent from './Models/ModelFormDialogContent';
import { Dialog, DialogContent } from './ui/dialog';

type NewModelDialogProps = {
  onSubmit: (model: NewModel) => void;
};

type EditModelDialogProps = {
  onSubmit: (model: Model) => void;
  onDelete: (model: Model) => void;
};

const NewModelFormDialogInner = ({
  onFormSubmit,
}: {
  onFormSubmit: (model: NewModel) => void;
}) => {
  return (
    <DialogContent className="flex max-h-screen flex-col">
      <ModelFormDialogContent.New
        provider={PROVIDER_OLLAMA}
        onFormSubmit={onFormSubmit}
      />
    </DialogContent>
  );
};

const NewModelFormDialog = forwardRef<DialogHandler<void>, NewModelDialogProps>(
  ({ onSubmit }, ref) => {
    const [showDialog, setShowDialog] = useState(false);

    useImperativeHandle(ref, () => ({
      open: () => {
        setShowDialog(true);
      },
      close: () => {
        setShowDialog(false);
      },
    }));

    const onFormSubmit = useCallback(
      (model: NewModel) => {
        onSubmit(model);
        setShowDialog(false);
      },
      [onSubmit]
    );

    return (
      <Dialog open={showDialog} onOpenChange={setShowDialog}>
        <NewModelFormDialogInner onFormSubmit={onFormSubmit} />
      </Dialog>
    );
  }
);

const EditModelFormDialog = forwardRef<
  DialogHandler<Model>,
  EditModelDialogProps
>(({ onSubmit, onDelete }, ref) => {
  const [showDialog, setShowDialog] = useState(false);
  const [model, setModel] = useState<Model>();

  useImperativeHandle(ref, () => ({
    open: (defaultValue?: Model) => {
      setModel(defaultValue);
      setShowDialog(true);
    },
    close: () => {
      setModel(undefined);
      setShowDialog(false);
    },
  }));

  const onFormSubmit = (updatedModel: Model) => {
    onSubmit(updatedModel);
    setShowDialog(false);
  };

  return model ? (
    <Dialog open={showDialog} onOpenChange={setShowDialog}>
      <DialogContent className="flex max-h-screen">
        <ModelFormDialogContent.Edit
          model={model}
          onFormSubmit={onFormSubmit}
          onDelete={onDelete}
        />
      </DialogContent>
    </Dialog>
  ) : null;
});

export default {
  New: NewModelFormDialog,
  Edit: EditModelFormDialog,
};
