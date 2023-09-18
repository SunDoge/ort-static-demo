import torch


class Add(torch.nn.Module):

    def forward(self, a, b):
        return a + b


torch.onnx.export(
    Add(),
    (torch.rand(3), torch.rand(3)),
    "add.onnx",
)
